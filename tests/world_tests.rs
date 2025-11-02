// World tests

use predator_prey_sim::simulation::world::World;
use predator_prey_sim::config::parameters::Parameters;

#[test]
fn test_world_creation() {
    let params = Parameters::default();
    let world = World::new(params);
    
    assert!(world.predator_count() > 0);
    assert!(world.prey_count() > 0);
}

#[test]
fn test_world_update() {
    let params = Parameters::default();
    let mut world = World::new(params);
    
    let initial_predator_count = world.predator_count();
    let initial_prey_count = world.prey_count();
    
    // Run a few updates
    for _ in 0..10 {
        world.update();
    }
    
    // Counts may change but shouldn't crash
    assert!(world.total_agents() > 0);
}

#[test]
fn test_world_reset() {
    let params = Parameters::default();
    let mut world = World::new(params.clone());
    
    let initial_predator_count = world.predator_count();
    let initial_prey_count = world.prey_count();
    
    // Run some updates
    for _ in 0..20 {
        world.update();
    }
    
    // Reset
    world.reset();
    
    // Should have same initial counts
    assert_eq!(world.predator_count(), initial_predator_count);
    assert_eq!(world.prey_count(), initial_prey_count);
}

#[test]
fn test_world_total_agents() {
    let params = Parameters::default();
    let world = World::new(params);
    
    let total = world.total_agents();
    assert_eq!(total, world.predator_count() + world.prey_count());
}

#[test]
fn test_world_max_agents_limit() {
    let mut params = Parameters::default();
    params.simulation.max_agents = 10; // Very low limit
    params.simulation.enable_reproduction = true;
    params.predator.initial_count = 5;
    params.prey.initial_count = 5;
    
    let mut world = World::new(params);
    
    // Run many updates to trigger reproduction
    for _ in 0..100 {
        world.update();
    }
    
    // Should not exceed max_agents
    assert!(world.total_agents() <= 10);
}

