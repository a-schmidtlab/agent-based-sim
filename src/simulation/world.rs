// World/environment management

use crate::simulation::agent::*;
use crate::simulation::predator::Predator;
use crate::simulation::prey::Prey;
use crate::config::parameters::*;
use crate::utils::math::{Vector2, distance_torus};

/// World manages all agents and the simulation environment
pub struct World {
    predators: Vec<Predator>,
    prey: Vec<Prey>,
    params: Parameters,
    next_id: AgentId,
}

impl World {
    /// Create a new world with the given parameters
    pub fn new(params: Parameters) -> Self {
        let mut world = Self {
            predators: Vec::new(),
            prey: Vec::new(),
            params,
            next_id: 1,
        };

        world.initialize_agents();
        world
    }

    /// Initialize agents according to parameters
    fn initialize_agents(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        // Spawn predators
        for _ in 0..self.params.predator.initial_count {
            let x = rng.gen_range(0.0..self.params.world.width);
            let y = rng.gen_range(0.0..self.params.world.height);
            let predator = Predator::new(
                self.next_id,
                Vector2::new(x, y),
                self.params.predator.clone(),
            );
            self.predators.push(predator);
            self.next_id += 1;
        }

        // Spawn prey
        for _ in 0..self.params.prey.initial_count {
            let x = rng.gen_range(0.0..self.params.world.width);
            let y = rng.gen_range(0.0..self.params.world.height);
            let prey = Prey::new(self.next_id, Vector2::new(x, y), self.params.prey.clone());
            self.prey.push(prey);
            self.next_id += 1;
        }
    }

    /// Update the world one simulation step
    pub fn update(&mut self) {
        // Build spatial index for efficient neighbor queries
        // For now, we'll use a simple approach and optimize later if needed
        
        // Prepare world state for each agent
        let world_state = self.build_world_state();

        // Update all predators
        let mut predator_actions = Vec::new();
        for (i, predator) in self.predators.iter_mut().enumerate() {
            let action = predator.update(&world_state);
            predator_actions.push((i, action));
        }

        // Update all prey
        let mut prey_actions = Vec::new();
        for (i, prey) in self.prey.iter_mut().enumerate() {
            let action = prey.update(&world_state);
            prey_actions.push((i, action));
        }

        // Process actions
        self.process_actions(predator_actions, prey_actions);

        // Remove dead agents
        self.predators.retain(|p| p.is_alive());
        self.prey.retain(|p| p.is_alive());

        // Limit total agents
        self.enforce_max_agents();
    }

    /// Build world state information for agents
    fn build_world_state(&self) -> WorldState {
        let mut nearby_predators = Vec::new();
        let mut nearby_prey = Vec::new();

        // Build lists of nearby agents for each agent
        // This is O(n²) but acceptable for moderate numbers of agents
        // Can be optimized with spatial partitioning later

        // Use torus distance for proper wraparound behavior
        let world_width = self.params.world.width;
        let world_height = self.params.world.height;

        for prey in &self.prey {
            // Find nearby predators
            for predator in &self.predators {
                let dist = distance_torus(&prey.position(), &predator.position(), world_width, world_height);
                if dist <= self.params.predator.perception_radius.max(self.params.prey.detection_radius) {
                    nearby_predators.push((predator.id(), predator.position(), dist));
                }
            }
        }

        for predator in &self.predators {
            // Find nearby prey
            for prey in &self.prey {
                let dist = distance_torus(&predator.position(), &prey.position(), world_width, world_height);
                if dist <= self.params.predator.perception_radius {
                    nearby_prey.push((prey.id(), prey.position(), dist));
                }
            }
        }

        WorldState {
            width: self.params.world.width,
            height: self.params.world.height,
            boundary_type: self.params.world.boundary_type,
            nearby_predators,
            nearby_prey,
            dt: self.params.simulation.dt,
        }
    }

    /// Process agent actions (consumption, reproduction, etc.)
    fn process_actions(
        &mut self,
        predator_actions: Vec<(usize, AgentAction)>,
        prey_actions: Vec<(usize, AgentAction)>,
    ) {
        // Process predator actions
        let mut consumed_prey_ids = Vec::new();
        let mut new_predators = Vec::new();

        for (_idx, action) in predator_actions {
            match action {
                AgentAction::Consumed { target_id } => {
                    consumed_prey_ids.push(target_id);
                }
                AgentAction::Reproduce { position, energy: _ } => {
                    if self.params.simulation.enable_reproduction {
                        let new_predator = Predator::new(
                            self.next_id,
                            position,
                            self.params.predator.clone(),
                        );
                        // Set energy manually (we'd need to expose this in BaseAgent)
                        // For now, new agents start with initial_energy from params
                        new_predators.push(new_predator);
                        self.next_id += 1;
                    }
                }
                _ => {}
            }
        }

        // Process prey actions
        let mut new_prey = Vec::new();
        for (_idx, action) in prey_actions {
            match action {
                AgentAction::Reproduce { position, energy: _ } => {
                    if self.params.simulation.enable_reproduction {
                        let new_prey_agent = Prey::new(
                            self.next_id,
                            position,
                            self.params.prey.clone(),
                        );
                        new_prey.push(new_prey_agent);
                        self.next_id += 1;
                    }
                }
                _ => {}
            }
        }

        // Remove consumed prey
        self.prey.retain(|p| !consumed_prey_ids.contains(&p.id()));

        // Add new agents
        self.predators.extend(new_predators);
        self.prey.extend(new_prey);
    }

    /// Enforce maximum agent limit
    fn enforce_max_agents(&mut self) {
        let total = self.predators.len() + self.prey.len();
        if total > self.params.simulation.max_agents as usize {
            // Remove oldest agents first (simple FIFO)
            let to_remove = total - self.params.simulation.max_agents as usize;
            
            // Remove predators first if needed
            if self.predators.len() > to_remove {
                self.predators.drain(0..to_remove);
            } else {
                let remove_prey = to_remove - self.predators.len();
                self.predators.clear();
                if self.prey.len() > remove_prey {
                    self.prey.drain(0..remove_prey);
                } else {
                    self.prey.clear();
                }
            }
        }
    }

    /// Get current predator count
    pub fn predator_count(&self) -> usize {
        self.predators.len()
    }

    /// Get current prey count
    pub fn prey_count(&self) -> usize {
        self.prey.len()
    }

    /// Get total agent count
    pub fn total_agents(&self) -> usize {
        self.predators.len() + self.prey.len()
    }

    /// Get all predators (for visualization)
    pub fn predators(&self) -> &[Predator] {
        &self.predators
    }

    /// Get all prey (for visualization)
    pub fn prey(&self) -> &[Prey] {
        &self.prey
    }

    /// Update parameters (useful for real-time adjustment)
    pub fn update_parameters(&mut self, params: Parameters) {
        self.params = params;
        // Note: Existing agents keep their current parameters
        // New agents will use the new parameters
    }

    /// Get current parameters
    pub fn parameters(&self) -> &Parameters {
        &self.params
    }

    /// Reset the world (clear all agents and reinitialize)
    pub fn reset(&mut self) {
        self.predators.clear();
        self.prey.clear();
        self.next_id = 1;
        self.initialize_agents();
    }

    /// Spawn additional predators at random positions
    pub fn spawn_predators(&mut self, count: u32) -> u32 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut spawned = 0;
        
        for _ in 0..count {
            // Check max agents limit
            if self.total_agents() >= self.params.simulation.max_agents as usize {
                break;
            }
            
            let x = rng.gen_range(0.0..self.params.world.width);
            let y = rng.gen_range(0.0..self.params.world.height);
            let predator = Predator::new(
                self.next_id,
                Vector2::new(x, y),
                self.params.predator.clone(),
            );
            self.predators.push(predator);
            self.next_id += 1;
            spawned += 1;
        }
        
        spawned
    }

    /// Spawn additional prey at random positions
    pub fn spawn_prey(&mut self, count: u32) -> u32 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut spawned = 0;
        
        for _ in 0..count {
            // Check max agents limit
            if self.total_agents() >= self.params.simulation.max_agents as usize {
                break;
            }
            
            let x = rng.gen_range(0.0..self.params.world.width);
            let y = rng.gen_range(0.0..self.params.world.height);
            let prey = Prey::new(self.next_id, Vector2::new(x, y), self.params.prey.clone());
            self.prey.push(prey);
            self.next_id += 1;
            spawned += 1;
        }
        
        spawned
    }

    /// Clear all agents
    pub fn clear_all(&mut self) {
        self.predators.clear();
        self.prey.clear();
    }

    /// Get average energy for predators
    pub fn average_predator_energy(&self) -> f64 {
        if self.predators.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.predators.iter().map(|p| p.energy()).sum();
        sum / self.predators.len() as f64
    }

    /// Get average energy for prey
    pub fn average_prey_energy(&self) -> f64 {
        if self.prey.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.prey.iter().map(|p| p.energy()).sum();
        sum / self.prey.len() as f64
    }
}
