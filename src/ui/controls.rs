// Sliders, buttons, parameter UI

use crate::config::parameters::*;
use crate::ui::visualization::VisualizationSettings;

/// UI state for controls
pub struct ControlPanel {
    pub params: Parameters,
    pub viz_settings: VisualizationSettings,
    pub paused: bool,
    pub speed_multiplier: f64,
    pub spawn_predators: u32,
    pub spawn_prey: u32,
    pub spawn_predators_requested: bool,
    pub spawn_prey_requested: bool,
}

impl Default for ControlPanel {
    fn default() -> Self {
        Self {
            params: Parameters::default(),
            viz_settings: VisualizationSettings::default(),
            paused: false,
            speed_multiplier: 1.0,
            spawn_predators: 10,
            spawn_prey: 50,
            spawn_predators_requested: false,
            spawn_prey_requested: false,
        }
    }
}

impl ControlPanel {
    /// Show the control panel with all sliders and buttons
    pub fn show(&mut self, ui: &mut egui::Ui) -> (bool, bool, bool, bool) {
        let mut reset_requested = false;
        self.spawn_predators_requested = false;
        self.spawn_prey_requested = false;
        
        // Control buttons
        ui.horizontal(|ui| {
            if ui.button(if self.paused { "▶ Resume" } else { "⏸ Pause" }).clicked() {
                self.paused = !self.paused;
            }
            
            if ui.button("🔄 Reset").clicked() {
                reset_requested = true;
            }
            
            if ui.button("🗑️ Clear All").clicked() {
                reset_requested = true; // Will clear via reset with 0 counts
            }
            
            if ui.button("💾 Save Preset").clicked() {
                // TODO: Implement save
            }
            
            if ui.button("📂 Load Preset").clicked() {
                // TODO: Implement load
            }
        });
        
        ui.separator();
        
        // Population spawning controls
        ui.label(egui::RichText::new("Population Controls").heading());
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Initial Predators:");
                let val = self.spawn_predators;
                ui.add(egui::Slider::new(&mut self.spawn_predators, 0..=500)
                    .text(format!("{}", val)));
                if ui.button("🐺 Spawn Now").clicked() {
                    self.spawn_predators_requested = true;
                }
            });
            
            ui.separator();
            
            ui.vertical(|ui| {
                ui.label("Initial Prey:");
                let val = self.spawn_prey;
                ui.add(egui::Slider::new(&mut self.spawn_prey, 0..=500)
                    .text(format!("{}", val)));
                if ui.button("🐰 Spawn Now").clicked() {
                    self.spawn_prey_requested = true;
                }
            });
        });
        
        ui.label(format!("Current: {} predators, {} prey", 
            self.params.predator.initial_count, 
            self.params.prey.initial_count));
        
        ui.separator();
        
        // Speed control
        ui.label("Simulation Speed:");
        let speed_value = self.speed_multiplier;
        ui.add(egui::Slider::new(&mut self.speed_multiplier, 0.1..=5.0)
            .text(format!("{:.1}x", speed_value)));
        
        ui.separator();
        
        let mut params_changed = false;
        
        // Collapsible sections
        egui::ScrollArea::vertical()
            .max_height(600.0)
            .show(ui, |ui| {
                params_changed |= self.show_predator_controls(ui);
                params_changed |= self.show_prey_controls(ui);
                params_changed |= self.show_world_controls(ui);
                params_changed |= self.show_simulation_controls(ui);
                params_changed |= self.show_visualization_controls(ui);
            });
        
        (params_changed, reset_requested, self.spawn_predators_requested, self.spawn_prey_requested)
    }
    
    fn show_predator_controls(&mut self, ui: &mut egui::Ui) -> bool {
        let _changed = false;
        
        egui::CollapsingHeader::new("🐺 Predator Parameters")
            .default_open(true)
            .show(ui, |ui| {
                ui.add(egui::Slider::new(&mut self.params.predator.initial_energy, 10.0..=500.0)
                    .text("Initial Energy"));
                
                ui.add(egui::Slider::new(&mut self.params.predator.max_speed, 0.5..=10.0)
                    .text("Max Speed"));
                
                ui.add(egui::Slider::new(&mut self.params.predator.perception_radius, 10.0..=200.0)
                    .text("Perception Radius"));
                
                ui.add(egui::Slider::new(&mut self.params.predator.capture_distance, 1.0..=20.0)
                    .text("Capture Distance"));
                
                ui.add(egui::Slider::new(&mut self.params.predator.energy_per_tick, 0.1..=5.0)
                    .text("Energy per Tick"));
                
                ui.add(egui::Slider::new(&mut self.params.predator.energy_gain_from_prey, 10.0..=200.0)
                    .text("Energy Gain from Prey"));
                
                ui.add(egui::Slider::new(&mut self.params.predator.reproduction_threshold, 50.0..=500.0)
                    .text("Reproduction Threshold"));
                
                ui.add(egui::Slider::new(&mut self.params.predator.reproduction_cost, 20.0..=200.0)
                    .text("Reproduction Cost"));
                
                ui.add(egui::Slider::new(&mut self.params.predator.initial_count, 0..=100)
                    .text("Initial Count"));
            });
        
        true // Parameters may have changed
    }
    
    fn show_prey_controls(&mut self, ui: &mut egui::Ui) -> bool {
        let _changed = false;
        
        egui::CollapsingHeader::new("🐰 Prey Parameters")
            .default_open(true)
            .show(ui, |ui| {
                ui.add(egui::Slider::new(&mut self.params.prey.initial_energy, 10.0..=500.0)
                    .text("Initial Energy"));
                
                ui.add(egui::Slider::new(&mut self.params.prey.max_speed, 0.5..=10.0)
                    .text("Max Speed"));
                
                ui.add(egui::Slider::new(&mut self.params.prey.detection_radius, 10.0..=200.0)
                    .text("Detection Radius"));
                
                ui.add(egui::Slider::new(&mut self.params.prey.flee_distance, 10.0..=100.0)
                    .text("Flee Distance"));
                
                ui.add(egui::Slider::new(&mut self.params.prey.energy_regeneration, 0.0..=2.0)
                    .text("Energy Regeneration"));
                
                ui.add(egui::Slider::new(&mut self.params.prey.energy_loss_fleeing, 0.0..=1.0)
                    .text("Energy Loss When Fleeing"));
                
                ui.add(egui::Slider::new(&mut self.params.prey.reproduction_threshold, 50.0..=500.0)
                    .text("Reproduction Threshold"));
                
                ui.add(egui::Slider::new(&mut self.params.prey.reproduction_cost, 20.0..=200.0)
                    .text("Reproduction Cost"));
                
                ui.add(egui::Slider::new(&mut self.params.prey.initial_count, 0..=200)
                    .text("Initial Count"));
            });
        
        true // Parameters may have changed
    }
    
    fn show_world_controls(&mut self, ui: &mut egui::Ui) -> bool {
        let _changed = false;
        
        egui::CollapsingHeader::new("🌍 World Parameters")
            .default_open(false)
            .show(ui, |ui| {
                ui.add(egui::Slider::new(&mut self.params.world.width, 200.0..=2000.0)
                    .text("Width"));
                
                ui.add(egui::Slider::new(&mut self.params.world.height, 200.0..=2000.0)
                    .text("Height"));
                
                ui.horizontal(|ui| {
                    ui.label("Boundary Type:");
                    ui.selectable_value(
                        &mut self.params.world.boundary_type,
                        BoundaryType::Wraparound,
                        "Wraparound",
                    );
                    ui.selectable_value(
                        &mut self.params.world.boundary_type,
                        BoundaryType::Walls,
                        "Walls",
                    );
                });
            });
        
        true // Parameters may have changed
    }
    
    fn show_simulation_controls(&mut self, ui: &mut egui::Ui) -> bool {
        let _changed = false;
        
        egui::CollapsingHeader::new("⚙️ Simulation Parameters")
            .default_open(false)
            .show(ui, |ui| {
                ui.add(egui::Slider::new(&mut self.params.simulation.tick_rate, 1.0..=120.0)
                    .text("Tick Rate (Hz)"));
                
                self.params.simulation.update_dt();
                
                ui.add(egui::Slider::new(&mut self.params.simulation.max_agents, 10..=5000)
                    .text("Max Agents"));
                
                ui.checkbox(&mut self.params.simulation.enable_reproduction, "Enable Reproduction");
            });
        
        true // Parameters may have changed
    }
    
    fn show_visualization_controls(&mut self, ui: &mut egui::Ui) -> bool {
        let _changed = false;
        
        egui::CollapsingHeader::new("👁️ Visualization")
            .default_open(false)
            .show(ui, |ui| {
                ui.add(egui::Slider::new(&mut self.viz_settings.agent_size, 2.0..=10.0)
                    .text("Agent Size"));
                
                ui.checkbox(&mut self.viz_settings.show_velocity_vectors, "Show Velocity Vectors");
                ui.checkbox(&mut self.viz_settings.show_perception_radius, "Show Perception Radius");
                ui.checkbox(&mut self.viz_settings.show_energy_colors, "Show Energy Colors");
                ui.checkbox(&mut self.viz_settings.grid_enabled, "Show Grid");
                
                if self.viz_settings.grid_enabled {
                    ui.add(egui::Slider::new(&mut self.viz_settings.grid_size, 10.0..=100.0)
                        .text("Grid Size"));
                }
            });
        
        true // Settings may have changed
    }
    
}
