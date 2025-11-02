// Canvas rendering of agents

use crate::simulation::world::World;
use crate::simulation::agent::{Agent, AgentType};
use crate::utils::color::Colors;
use crate::utils::math::Vector2;

/// Visualization settings
pub struct VisualizationSettings {
    pub show_velocity_vectors: bool,
    pub show_perception_radius: bool,
    pub show_energy_colors: bool,
    pub agent_size: f32,
    pub grid_enabled: bool,
    pub grid_size: f32,
}

impl Default for VisualizationSettings {
    fn default() -> Self {
        Self {
            show_velocity_vectors: false,
            show_perception_radius: false,
            show_energy_colors: true,
            agent_size: 4.0,
            grid_enabled: false,
            grid_size: 20.0,
        }
    }
}

/// Render the simulation world to an egui painter
pub fn render_world(
    painter: &egui::Painter,
    world: &World,
    settings: &VisualizationSettings,
    canvas_rect: egui::Rect,
) {
    let world_params = world.parameters();
    let world_width = world_params.world.width;
    let world_height = world_params.world.height;
    
    let scale_x = canvas_rect.width() / world_width as f32;
    let scale_y = canvas_rect.height() / world_height as f32;
    
    // Helper to convert world coordinates to screen coordinates
    let to_screen = |pos: Vector2| -> egui::Pos2 {
        egui::Pos2::new(
            canvas_rect.left() + pos.x as f32 * scale_x,
            canvas_rect.top() + pos.y as f32 * scale_y,
        )
    };
    
    // Draw grid if enabled
    if settings.grid_enabled {
        draw_grid(painter, canvas_rect, settings.grid_size);
    }
    
    // Draw predators
    for predator in world.predators() {
        render_agent(
            painter,
            predator,
            AgentType::Predator,
            world_params,
            &to_screen,
            settings,
        );
    }
    
    // Draw prey
    for prey in world.prey() {
        render_agent(
            painter,
            prey,
            AgentType::Prey,
            world_params,
            &to_screen,
            settings,
        );
    }
}

/// Render a single agent
fn render_agent(
    painter: &egui::Painter,
    agent: &dyn Agent,
    agent_type: AgentType,
    params: &crate::config::parameters::Parameters,
    to_screen: &dyn Fn(Vector2) -> egui::Pos2,
    settings: &VisualizationSettings,
) {
    let pos = agent.position();
    let screen_pos = to_screen(pos);
    
    // Determine color based on energy if enabled
    let color = if settings.show_energy_colors {
        let max_energy = match agent_type {
            AgentType::Predator => params.predator.initial_energy,
            AgentType::Prey => params.prey.initial_energy,
        };
        let energy_factor = (agent.energy() / max_energy).min(1.0).max(0.0);
        Colors::energy_color(energy_factor, agent_type == AgentType::Predator)
    } else {
        match agent_type {
            AgentType::Predator => Colors::predator(),
            AgentType::Prey => Colors::prey(),
        }
    };
    
    let egui_color = color.to_egui_color32();
    
    // Draw agent as circle
    painter.circle_filled(screen_pos, settings.agent_size, egui_color);
    
    // Draw velocity vector if enabled
    if settings.show_velocity_vectors {
        let vel = agent.velocity();
        let vel_length = vel.magnitude() as f32;
        if vel_length > 0.01 {
            let end_pos = to_screen(pos.add(&vel.scale(0.1))); // Scale down for visibility
            painter.line_segment(
                [screen_pos, end_pos],
                egui::Stroke::new(1.0, egui_color),
            );
            
            // Draw arrow head
            let angle = vel.y.atan2(vel.x) as f32;
            let arrow_len = 5.0;
            let arrow_angle = std::f32::consts::PI / 6.0;
            let arrow1 = egui::Pos2::new(
                end_pos.x - arrow_len * (angle - arrow_angle).cos(),
                end_pos.y - arrow_len * (angle - arrow_angle).sin(),
            );
            let arrow2 = egui::Pos2::new(
                end_pos.x - arrow_len * (angle + arrow_angle).cos(),
                end_pos.y - arrow_len * (angle + arrow_angle).sin(),
            );
            painter.line_segment([end_pos, arrow1], egui::Stroke::new(1.0, egui_color));
            painter.line_segment([end_pos, arrow2], egui::Stroke::new(1.0, egui_color));
        }
    }
    
    // Draw perception/detection radius if enabled
    if settings.show_perception_radius {
        let radius = match agent_type {
            AgentType::Predator => params.predator.perception_radius,
            AgentType::Prey => params.prey.detection_radius,
        } as f32;
        
        let screen_radius = radius * ((screen_pos.x - screen_pos.x) / 100.0).max(1.0) * 0.5; // Approximate scaling
        
        painter.circle_stroke(
            screen_pos,
            screen_radius * 0.1, // Scale down for visibility
            egui::Stroke::new(1.0, egui_color.linear_multiply(0.3)),
        );
    }
}

/// Draw grid overlay
fn draw_grid(painter: &egui::Painter, rect: egui::Rect, grid_size: f32) {
    let color = Colors::grid().to_egui_color32();
    let stroke = egui::Stroke::new(0.5, color);
    
    // Vertical lines
    let mut x = rect.left();
    while x <= rect.right() {
        painter.line_segment(
            [egui::Pos2::new(x, rect.top()), egui::Pos2::new(x, rect.bottom())],
            stroke,
        );
        x += grid_size;
    }
    
    // Horizontal lines
    let mut y = rect.top();
    while y <= rect.bottom() {
        painter.line_segment(
            [egui::Pos2::new(rect.left(), y), egui::Pos2::new(rect.right(), y)],
            stroke,
        );
        y += grid_size;
    }
}

/// Render statistics overlay
pub fn render_statistics(
    ui: &mut egui::Ui,
    world: &World,
) {
    egui::Window::new("Statistics")
        .collapsible(true)
        .resizable(true)
        .default_size([200.0, 150.0])
        .show(ui.ctx(), |ui| {
            ui.label(format!("Predators: {}", world.predator_count()));
            ui.label(format!("Prey: {}", world.prey_count()));
            ui.label(format!("Total Agents: {}", world.total_agents()));
            
            ui.separator();
            
            // Average energy (would need to calculate from agents)
            ui.label("Energy Levels:");
            ui.label(format!("  Predators: {:.1}", 100.0)); // Placeholder
            ui.label(format!("  Prey: {:.1}", 80.0)); // Placeholder
        });
}
