// Prey agent implementation

use crate::simulation::agent::*;
use crate::config::parameters::PreyParameters;
use crate::utils::math::{Vector2, from_angle};

/// Prey agent
#[derive(Debug, Clone)]
pub struct Prey {
    base: BaseAgent,
    params: PreyParameters,
}

impl Prey {
    /// Create a new prey agent
    pub fn new(id: AgentId, position: Vector2, params: PreyParameters) -> Self {
        let base = BaseAgent::new(
            id,
            AgentType::Prey,
            position,
            params.initial_energy,
            params.max_speed,
        );

        Self { base, params }
    }

    /// Find the nearest predator within detection radius
    fn find_nearest_predator(&self, world_state: &WorldState) -> Option<(AgentId, Vector2, f64)> {
        world_state
            .nearby_predators
            .iter()
            .min_by(|(_, _, d1), (_, _, d2)| d1.partial_cmp(d2).unwrap())
            .copied()
    }

    /// Calculate flee velocity away from a threat
    fn flee(&self, threat: Vector2) -> Vector2 {
        let away = self.base.position.subtract(&threat);
        let distance = away.magnitude();

        if distance > 0.0 {
            // Normalize and scale by max speed
            away.normalize().scale(self.base.max_speed)
        } else {
            // If at same position, move in random direction
            use rand::Rng;
            let mut rng = rand::thread_rng();
            from_angle(rng.gen::<f64>() * std::f64::consts::PI * 2.0, self.base.max_speed)
        }
    }
}

impl Agent for Prey {
    fn id(&self) -> AgentId {
        self.base.id
    }

    fn agent_type(&self) -> AgentType {
        AgentType::Prey
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
        // Regenerate energy naturally
        self.base.add_energy(self.params.energy_regeneration * world_state.dt);
        self.base.increment_age();

        // If dead, no action
        if !self.base.check_alive() {
            return AgentAction::None;
        }

        // Check for nearby predators
        if let Some((_, predator_pos, distance)) = self.find_nearest_predator(world_state) {
            // If predator is within flee distance, flee
            if distance <= self.params.flee_distance {
                let flee_velocity = self.flee(predator_pos);
                self.base.set_velocity(flee_velocity);
                // Extra energy loss when fleeing
                self.base.consume_energy(self.params.energy_loss_fleeing * world_state.dt);
            } else {
                // Predator nearby but not immediate threat - slow movement
                self.base.set_velocity(self.base.velocity.scale(0.9));
            }
        } else {
            // No predators nearby - can move more freely (simple wander or slow down)
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
