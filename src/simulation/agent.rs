// Base agent trait and implementation

use crate::utils::math::Vector2;

/// Unique identifier for agents
pub type AgentId = u32;

/// Agent type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentType {
    Predator,
    Prey,
}

/// Base agent trait - all agents must implement this
pub trait Agent {
    /// Get the agent's unique ID
    fn id(&self) -> AgentId;

    /// Get the agent's type
    fn agent_type(&self) -> AgentType;

    /// Get the agent's current position
    fn position(&self) -> Vector2;

    /// Get the agent's current velocity
    fn velocity(&self) -> Vector2;

    /// Get the agent's current energy level
    fn energy(&self) -> f64;

    /// Check if the agent is alive
    fn is_alive(&self) -> bool;

    /// Get the agent's age (number of ticks lived)
    fn age(&self) -> u32;

    /// Update the agent - called each simulation tick
    fn update(&mut self, world_state: &WorldState) -> AgentAction;

    /// Get the agent's maximum speed
    fn max_speed(&self) -> f64;
}

/// Information about the world state that agents can query
#[derive(Debug, Clone)]
pub struct WorldState {
    /// Width of the world
    pub width: f64,
    /// Height of the world
    pub height: f64,
    /// Boundary type (wraparound or walls)
    pub boundary_type: crate::config::parameters::BoundaryType,
    /// Nearby predators (within perception range)
    pub nearby_predators: Vec<(AgentId, Vector2, f64)>, // (id, position, distance)
    /// Nearby prey (within perception range)
    pub nearby_prey: Vec<(AgentId, Vector2, f64)>, // (id, position, distance)
    /// Delta time (time step)
    pub dt: f64,
}

/// Action that an agent can take during an update
#[derive(Debug, Clone)]
pub enum AgentAction {
    /// No special action, just move normally
    None,
    /// Agent wants to move to a new position (with new velocity)
    Move { position: Vector2, velocity: Vector2 },
    /// Agent consumed another agent (predator eating prey)
    Consumed { target_id: AgentId },
    /// Agent wants to reproduce (spawn new agent)
    Reproduce { position: Vector2, energy: f64 },
}

/// Base agent data structure
#[derive(Debug, Clone)]
pub struct BaseAgent {
    pub id: AgentId,
    pub agent_type: AgentType,
    pub position: Vector2,
    pub velocity: Vector2,
    pub energy: f64,
    pub age: u32,
    pub max_speed: f64,
}

impl BaseAgent {
    /// Create a new base agent
    pub fn new(
        id: AgentId,
        agent_type: AgentType,
        position: Vector2,
        initial_energy: f64,
        max_speed: f64,
    ) -> Self {
        Self {
            id,
            agent_type,
            position,
            velocity: Vector2::zero(),
            energy: initial_energy,
            age: 0,
            max_speed,
        }
    }

    /// Update the agent's position based on velocity
    pub fn update_position(&mut self, world_state: &WorldState) {
        let dt = world_state.dt;
        let new_position = self.position.add(&self.velocity.scale(dt));

        // Apply boundary conditions
        self.position = match world_state.boundary_type {
            crate::config::parameters::BoundaryType::Wraparound => {
                crate::utils::math::wrap_position(new_position, world_state.width, world_state.height)
            }
            crate::config::parameters::BoundaryType::Walls => {
                crate::utils::math::clamp_position(new_position, world_state.width, world_state.height)
            }
        };
    }

    /// Set the velocity, ensuring it doesn't exceed max_speed
    pub fn set_velocity(&mut self, velocity: Vector2) {
        self.velocity = velocity.limit(self.max_speed);
    }

    /// Reduce energy by the given amount
    pub fn consume_energy(&mut self, amount: f64) {
        self.energy = (self.energy - amount).max(0.0);
    }

    /// Add energy
    pub fn add_energy(&mut self, amount: f64) {
        self.energy += amount;
    }

    /// Increment age
    pub fn increment_age(&mut self) {
        self.age += 1;
    }

    /// Check if agent is alive (has energy > 0)
    pub fn check_alive(&self) -> bool {
        self.energy > 0.0
    }
}
