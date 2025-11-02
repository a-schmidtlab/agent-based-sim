// Predator agent implementation

use crate::simulation::agent::*;
use crate::config::parameters::PredatorParameters;
use crate::utils::math::{Vector2, distance, from_angle};

/// Predator agent
#[derive(Debug, Clone)]
pub struct Predator {
    base: BaseAgent,
    params: PredatorParameters,
}

impl Predator {
    /// Create a new predator agent
    pub fn new(id: AgentId, position: Vector2, params: PredatorParameters) -> Self {
        let base = BaseAgent::new(
            id,
            AgentType::Predator,
            position,
            params.initial_energy,
            params.max_speed,
        );

        Self { base, params }
    }

    /// Find the nearest prey within perception radius
    fn find_nearest_prey(&self, world_state: &WorldState) -> Option<(AgentId, Vector2, f64)> {
        world_state
            .nearby_prey
            .iter()
            .min_by(|(_, _, d1), (_, _, d2)| d1.partial_cmp(d2).unwrap())
            .copied()
    }

    /// Calculate steering force toward a target
    fn seek(&self, target: Vector2) -> Vector2 {
        let desired = target.subtract(&self.base.position);
        let distance = desired.magnitude();

        if distance > 0.0 {
            // Normalize and scale by max speed
            desired.normalize().scale(self.base.max_speed)
        } else {
            Vector2::zero()
        }
    }
}

impl Agent for Predator {
    fn id(&self) -> AgentId {
        self.base.id
    }

    fn agent_type(&self) -> AgentType {
        AgentType::Predator
    }

    fn position(&self) -> Vector2 {
        self.base.position
    }

    fn velocity(&self) -> Vector2 {
        self.base.velocity
    }

    fn energy(&self) -> f64 {
        self.base.energy
    }

    fn is_alive(&self) -> bool {
        self.base.check_alive()
    }

    fn age(&self) -> u32 {
        self.base.age
    }

    fn max_speed(&self) -> f64 {
        self.base.max_speed
    }

    fn update(&mut self, world_state: &WorldState) -> AgentAction {
        // Consume energy each tick
        self.base.consume_energy(self.params.energy_per_tick * world_state.dt);
        self.base.increment_age();

        // If dead, no action
        if !self.base.check_alive() {
            return AgentAction::None;
        }

        // Try to find and chase nearest prey
        if let Some((prey_id, prey_pos, distance)) = self.find_nearest_prey(world_state) {
            // If within capture distance, consume the prey
            if distance <= self.params.capture_distance {
                self.base.add_energy(self.params.energy_gain_from_prey);
                return AgentAction::Consumed { target_id: prey_id };
            }

            // Otherwise, move toward the prey
            let desired_velocity = self.seek(prey_pos);
            self.base.set_velocity(desired_velocity);
        } else {
            // No prey nearby - random wander or slow down
            // Simple implementation: slow down
            self.base.set_velocity(self.base.velocity.scale(0.95));
        }

        // Check for reproduction
        if self.base.energy >= self.params.reproduction_threshold {
            // Spawn near current position with some random offset
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let angle = rng.gen::<f64>() * std::f64::consts::PI * 2.0;
            let distance = rng.gen::<f64>() * 20.0;
            let offset = from_angle(angle, distance);
            let spawn_pos = self.base.position.add(&offset);
            
            // Clamp spawn position to world bounds
            let spawn_pos = Vector2 {
                x: spawn_pos.x.max(10.0).min(world_state.width - 10.0),
                y: spawn_pos.y.max(10.0).min(world_state.height - 10.0),
            };
            
            self.base.consume_energy(self.params.reproduction_cost);
            return AgentAction::Reproduce {
                position: spawn_pos,
                energy: self.params.initial_energy,
            };
        }

        // Update position
        self.base.update_position(world_state);

        AgentAction::None
    }
}

// Add fastrand dependency for random numbers
// For now, we'll use a simple approach. We'll need to add fastrand to Cargo.toml
// Or we can use a seeded RNG. Let me check if we can use rand instead.
