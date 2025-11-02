// Simulation parameters struct

use serde::{Deserialize, Serialize};

/// Predator-specific parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredatorParameters {
    pub initial_energy: f64,
    pub max_speed: f64,
    pub perception_radius: f64,
    pub capture_distance: f64,
    pub energy_per_tick: f64,
    pub energy_gain_from_prey: f64,
    pub reproduction_threshold: f64,
    pub reproduction_cost: f64,
    pub initial_count: u32,
}

impl Default for PredatorParameters {
    fn default() -> Self {
        Self {
            initial_energy: 100.0,
            max_speed: 2.0,
            perception_radius: 50.0,
            capture_distance: 5.0,
            energy_per_tick: 0.5,
            energy_gain_from_prey: 50.0,
            reproduction_threshold: 150.0,
            reproduction_cost: 80.0,
            initial_count: 10,
        }
    }
}

/// Prey-specific parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreyParameters {
    pub initial_energy: f64,
    pub max_speed: f64,
    pub detection_radius: f64,
    pub flee_distance: f64,
    pub energy_regeneration: f64,
    pub energy_loss_fleeing: f64,
    pub reproduction_threshold: f64,
    pub reproduction_cost: f64,
    pub initial_count: u32,
}

impl Default for PreyParameters {
    fn default() -> Self {
        Self {
            initial_energy: 80.0,
            max_speed: 2.5,
            detection_radius: 60.0,
            flee_distance: 40.0,
            energy_regeneration: 0.3,
            energy_loss_fleeing: 0.2,
            reproduction_threshold: 120.0,
            reproduction_cost: 60.0,
            initial_count: 50,
        }
    }
}

/// World/environment parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldParameters {
    pub width: f64,
    pub height: f64,
    pub boundary_type: BoundaryType,
    pub food_spawn_rate: f64,
    pub food_energy: f64,
    pub enable_food: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BoundaryType {
    Wraparound,
    Walls,
}

impl Default for WorldParameters {
    fn default() -> Self {
        Self {
            width: 800.0,
            height: 600.0,
            boundary_type: BoundaryType::Wraparound, // Torus topography (default)
            food_spawn_rate: 0.01,
            food_energy: 20.0,
            enable_food: false,
        }
    }
}

/// Simulation control parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationParameters {
    pub tick_rate: f64, // Updates per second
    pub max_agents: u32,
    pub enable_reproduction: bool,
    pub dt: f64, // Delta time (usually 1.0 / tick_rate)
}

impl Default for SimulationParameters {
    fn default() -> Self {
        Self {
            tick_rate: 60.0,
            max_agents: 1000,
            enable_reproduction: true,
            dt: 1.0 / 60.0,
        }
    }
}

impl SimulationParameters {
    pub fn update_dt(&mut self) {
        self.dt = 1.0 / self.tick_rate;
    }
}

/// Complete simulation parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameters {
    pub predator: PredatorParameters,
    pub prey: PreyParameters,
    pub world: WorldParameters,
    pub simulation: SimulationParameters,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            predator: PredatorParameters::default(),
            prey: PreyParameters::default(),
            world: WorldParameters::default(),
            simulation: SimulationParameters::default(),
        }
    }
}

impl Parameters {
    /// Validate parameters to ensure they make sense
    pub fn validate(&self) -> Result<(), String> {
        if self.world.width <= 0.0 || self.world.height <= 0.0 {
            return Err("World dimensions must be positive".to_string());
        }

        if self.predator.max_speed <= 0.0 || self.prey.max_speed <= 0.0 {
            return Err("Agent speeds must be positive".to_string());
        }

        if self.predator.initial_energy <= 0.0 || self.prey.initial_energy <= 0.0 {
            return Err("Initial energy must be positive".to_string());
        }

        if self.simulation.tick_rate <= 0.0 {
            return Err("Tick rate must be positive".to_string());
        }

        Ok(())
    }

    /// Save parameters to a TOML file
    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let toml_string = toml::to_string_pretty(self)?;
        std::fs::write(path, toml_string)?;
        Ok(())
    }

    /// Load parameters from a TOML file
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let params: Parameters = toml::from_str(&content)?;
        params.validate()?;
        Ok(params)
    }
}
