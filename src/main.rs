// Predator-Prey Simulation - Main Entry Point

use eframe::egui;
use predator_prey_sim::simulation::world::World;
use predator_prey_sim::config::parameters::Parameters;
use predator_prey_sim::ui::controls::ControlPanel;
use predator_prey_sim::ui::visualization::render_world;
use predator_prey_sim::ui::statistics::{StatisticsCollector, render_population_graph, Statistics};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Predator-Prey Simulation"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Predator-Prey Simulation",
        options,
        Box::new(|_cc| Box::new(PredatorPreyApp::default())),
    )
}

struct PredatorPreyApp {
    world: World,
    control_panel: ControlPanel,
    layout: predator_prey_sim::ui::layout::UILayout,
    last_update_time: f64,
    reset_requested: bool,
    statistics: StatisticsCollector,
    show_graph: bool,
}

impl Default for PredatorPreyApp {
    fn default() -> Self {
        let params = Parameters::default();
        let world = World::new(params.clone());
        
        Self {
            world,
            control_panel: ControlPanel::default(),
            layout: predator_prey_sim::ui::layout::UILayout::default(),
            last_update_time: 0.0,
            reset_requested: false,
            statistics: StatisticsCollector::new(1000), // Keep last 1000 data points
            show_graph: true,
        }
    }
}

impl eframe::App for PredatorPreyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle reset
        if self.reset_requested {
            let mut params = self.control_panel.params.clone();
            // Use slider values for initial counts
            params.predator.initial_count = self.control_panel.spawn_predators;
            params.prey.initial_count = self.control_panel.spawn_prey;
            self.world = World::new(params);
            self.reset_requested = false;
            self.statistics.clear();
        }
        
        // Handle spawn requests
        if self.control_panel.spawn_predators_requested {
            let _spawned = self.world.spawn_predators(self.control_panel.spawn_predators);
            // Could show a message if spawned < requested
        }
        if self.control_panel.spawn_prey_requested {
            let _spawned = self.world.spawn_prey(self.control_panel.spawn_prey);
        }
        
        // Update simulation
        if !self.control_panel.paused {
            let current_time = ctx.input(|i| i.time);
            let dt = current_time - self.last_update_time;
            
            // Update multiple times per frame based on tick rate and speed multiplier
            let effective_dt = dt * self.control_panel.speed_multiplier;
            let tick_rate = self.control_panel.params.simulation.tick_rate;
            let ticks_per_frame = (effective_dt * tick_rate).max(0.0).min(10.0) as u32; // Cap at 10 ticks per frame
            
            for _ in 0..ticks_per_frame {
                self.world.update();
            }
            
            self.last_update_time = current_time;
            
            // Record statistics
            self.statistics.record(
                self.world.predator_count(),
                self.world.prey_count(),
                self.world.average_predator_energy(),
                self.world.average_prey_energy(),
            );
        }
        
        // Update parameters if changed
        self.world.update_parameters(self.control_panel.params.clone());
        
        // Main UI
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Predator-Prey Simulation");
            
            // Get available space for canvas
            let available_rect = ui.available_rect_before_wrap();
            let canvas_rect = egui::Rect::from_min_max(
                available_rect.min,
                available_rect.max,
            );
            
            // Create a painter for the canvas
            let painter = ui.painter();
            
            // Draw background
            painter.rect_filled(
                canvas_rect,
                0.0,
                predator_prey_sim::utils::color::Colors::background().to_egui_color32(),
            );
            
            // Render the simulation
            render_world(
                &painter,
                &self.world,
                &self.control_panel.viz_settings,
                canvas_rect,
            );
            
            // Handle canvas interactions (if needed)
            let response = ui.allocate_rect(canvas_rect, egui::Sense::click());
            if response.clicked() {
                // Could handle clicking on agents, spawning, etc.
            }
        });
        
        // Control panel side panel
        egui::SidePanel::right("controls")
            .resizable(true)
            .default_width(self.layout.control_panel_width)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Controls");
                });
                
                ui.separator();
                
                let (_params_changed, reset_requested, spawn_pred, spawn_prey) = self.control_panel.show(ui);
                if reset_requested {
                    self.reset_requested = true;
                }
                if spawn_pred {
                    self.control_panel.spawn_predators_requested = true;
                }
                if spawn_prey {
                    self.control_panel.spawn_prey_requested = true;
                }
            });
        
        // Statistics window (floating)
        if self.layout.show_statistics {
            egui::Window::new("Statistics")
                .collapsible(true)
                .resizable(true)
                .default_pos([10.0, 10.0])
                .default_size([250.0, 300.0])
                .show(ctx, |ui| {
                    ui.label(egui::RichText::new("Current Values").heading());
                    ui.label(format!("Predators: {}", self.world.predator_count()));
                    ui.label(format!("Prey: {}", self.world.prey_count()));
                    ui.label(format!("Total Agents: {}", self.world.total_agents()));
                    
                    ui.separator();
                    
                    ui.label(egui::RichText::new("Energy Levels").heading());
                    ui.label(format!("Predator Avg: {:.1}", self.world.average_predator_energy()));
                    ui.label(format!("Prey Avg: {:.1}", self.world.average_prey_energy()));
                    
                    ui.separator();
                    
                    let stats = self.statistics.stats();
                    if stats.data_points > 0 {
                        ui.label(egui::RichText::new("Historical Stats").heading());
                        ui.label(format!("Avg Predators: {:.1}", stats.average_predators));
                        ui.label(format!("Avg Prey: {:.1}", stats.average_prey));
                        ui.label(format!("Max Predators: {}", stats.max_predators));
                        ui.label(format!("Max Prey: {}", stats.max_prey));
                        ui.label(format!("Data Points: {}", stats.data_points));
                    }
                    
                    ui.separator();
                    
                    ui.label(egui::RichText::new("Simulation").heading());
                    ui.label(format!("Tick Rate: {:.1} Hz", self.control_panel.params.simulation.tick_rate));
                    ui.label(format!("Speed: {:.1}x", self.control_panel.speed_multiplier));
                    ui.label(format!("Status: {}", if self.control_panel.paused { "Paused" } else { "Running" }));
                    
                    ui.separator();
                    
                    if ui.button("Clear Statistics").clicked() {
                        self.statistics.clear();
                    }
                });
        }
        
        // Population graph window
        if self.show_graph {
            egui::Window::new("Population Graph")
                .collapsible(true)
                .resizable(true)
                .default_pos([270.0, 10.0])
                .default_size([600.0, 300.0])
                .show(ctx, |ui| {
                    ui.label("Population Over Time");
                    
                    let available = ui.available_rect_before_wrap();
                    let graph_rect = egui::Rect::from_min_max(
                        egui::Pos2::new(available.min.x, available.min.y + 30.0),
                        egui::Pos2::new(available.max.x, available.max.y),
                    );
                    
                    let painter = ui.painter();
                    render_population_graph(&painter, &self.statistics, graph_rect);
                    
                    ui.allocate_rect(graph_rect, egui::Sense::click());
                });
        }
        
        // Request repaint for animation
        ctx.request_repaint();
    }
}
