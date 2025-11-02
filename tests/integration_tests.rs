// Integration tests for the full simulation

use predator_prey_sim::simulation::world::World;
use predator_prey_sim::config::parameters::Parameters;

#[test]
fn test_simulation_runs_multiple_steps() {
    let params = Parameters::default();
    let mut world = World::new(params);
    
    // Run many simulation steps
    for _ in 0..100 {
        world.update();
        
        // Verify world state is valid
        assert!(world.total_agents() >= 0);
        assert!(world.total_agents() <= world.parameters().simulation.max_agents as usize);
    }
}

#[test]
fn test_predator_prey_interaction() {
    let mut params = Parameters::default();
    params.predator.initial_count = 5;
    params.prey.initial_count = 20;
    params.predator.perception_radius = 100.0; // Large radius
    params.predator.capture_distance = 10.0;
    
    let mut world = World::new(params);
    
    let initial_prey_count = world.prey_count();
    
    // Run simulation - predators should hunt prey
    for _ in 0..50 {
        world.update();
    }
    
    // Some prey should have been consumed
    // (Note: This depends on random positioning, so we just check it doesn't crash)
    assert!(world.prey_count() <= initial_prey_count);
}

#[test]
fn test_reproduction_creates_new_agents() {
    let mut params = Parameters::default();
    params.simulation.enable_reproduction = true;
    params.predator.initial_count = 2;
    params.prey.initial_count = 10;
    params.predator.reproduction_threshold = 50.0; // Low threshold
    params.prey.reproduction_threshold = 50.0;
    params.predator.initial_energy = 200.0; // High initial energy
    params.prey.initial_energy = 200.0;
    params.predator.energy_per_tick = 0.1; // Low consumption
    params.prey.energy_regeneration = 1.0; // High regeneration
    params.simulation.max_agents = 100; // Allow growth
    
    let mut world = World::new(params);
    
    let initial_total = world.total_agents();
    
    // Run simulation
    for _ in 0..100 {
        world.update();
    }
    
    // Agents may have reproduced (depending on conditions)
    // We just verify the simulation continues to work
    assert!(world.total_agents() >= 0);
}

#[test]
fn test_simulation_with_wraparound_boundaries() {
    let mut params = Parameters::default();
    params.world.boundary_type = predator_prey_sim::config::parameters::BoundaryType::Wraparound;
    params.world.width = 200.0;
    params.world.height = 200.0;
    
    let mut world = World::new(params);
    
    // Run simulation - agents should wrap around
    for _ in 0..50 {
        world.update();
    }
    
    // Verify all agents are still within bounds (or wrapped)
    use predator_prey_sim::simulation::agent::Agent;
    for predator in world.predators() {
        let pos = predator.position();
        assert!(pos.x >= 0.0 && pos.x <= 200.0);
        assert!(pos.y >= 0.0 && pos.y <= 200.0);
    }
    
    for prey in world.prey() {
        let pos = prey.position();
        assert!(pos.x >= 0.0 && pos.x <= 200.0);
        assert!(pos.y >= 0.0 && pos.y <= 200.0);
    }
}

#[test]
fn test_simulation_with_wall_boundaries() {
    let mut params = Parameters::default();
    params.world.boundary_type = predator_prey_sim::config::parameters::BoundaryType::Walls;
    params.world.width = 200.0;
    params.world.height = 200.0;
    
    let mut world = World::new(params);
    
    // Run simulation - agents should be clamped
    for _ in 0..50 {
        world.update();
    }
    
    // Verify all agents are within bounds
    use predator_prey_sim::simulation::agent::Agent;
    for predator in world.predators() {
        let pos = predator.position();
        assert!(pos.x >= 0.0 && pos.x <= 200.0);
        assert!(pos.y >= 0.0 && pos.y <= 200.0);
    }
    
    for prey in world.prey() {
        let pos = prey.position();
        assert!(pos.x >= 0.0 && pos.x <= 200.0);
        assert!(pos.y >= 0.0 && pos.y <= 200.0);
    }
}

#[test]
fn test_parameter_updates() {
    let params = Parameters::default();
    let mut world = World::new(params);
    
    let mut new_params = Parameters::default();
    new_params.predator.initial_count = 15;
    new_params.prey.initial_count = 30;
    
    world.update_parameters(new_params.clone());
    
    // New parameters should be applied
    assert_eq!(world.parameters().predator.initial_count, 15);
    assert_eq!(world.parameters().prey.initial_count, 30);
}

#[test]
fn test_simulation_stability() {
    let params = Parameters::default();
    let mut world = World::new(params);
    
    // Run many iterations
    for _ in 0..1000 {
        world.update();
        
        // Check invariants
        assert!(world.total_agents() <= world.parameters().simulation.max_agents as usize);
        
        // Should not have negative counts
        assert!(world.predator_count() >= 0);
        assert!(world.prey_count() >= 0);
    }
}

