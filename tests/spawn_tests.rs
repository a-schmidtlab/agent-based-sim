// Spawn functionality tests

use predator_prey_sim::simulation::world::World;
use predator_prey_sim::config::parameters::Parameters;

#[test]
fn test_spawn_predators() {
    let params = Parameters::default();
    let mut world = World::new(params);
    
    let initial_count = world.predator_count();
    let spawned = world.spawn_predators(5);
    
    assert_eq!(spawned, 5);
    assert_eq!(world.predator_count(), initial_count + 5);
}

#[test]
fn test_spawn_prey() {
    let params = Parameters::default();
    let mut world = World::new(params);
    
    let initial_count = world.prey_count();
    let spawned = world.spawn_prey(10);
    
    assert_eq!(spawned, 10);
    assert_eq!(world.prey_count(), initial_count + 10);
}

#[test]
fn test_spawn_respects_max_agents() {
    let mut params = Parameters::default();
    params.simulation.max_agents = 20;
    params.predator.initial_count = 10;
    params.prey.initial_count = 10;
    
    let mut world = World::new(params);
    
    // Already at max
    assert_eq!(world.total_agents(), 20);
    
    // Try to spawn more
    let spawned = world.spawn_predators(10);
    assert_eq!(spawned, 0); // Should not spawn any
    assert_eq!(world.total_agents(), 20);
}

#[test]
fn test_clear_all() {
    let params = Parameters::default();
    let mut world = World::new(params);
    
    assert!(world.predator_count() > 0);
    assert!(world.prey_count() > 0);
    
    world.clear_all();
    
    assert_eq!(world.predator_count(), 0);
    assert_eq!(world.prey_count(), 0);
}

#[test]
fn test_average_energies() {
    let params = Parameters::default();
    let world = World::new(params);
    
    // Should return valid averages (0 if empty, otherwise > 0)
    let pred_energy = world.average_predator_energy();
    let prey_energy = world.average_prey_energy();
    
    assert!(pred_energy >= 0.0);
    assert!(prey_energy >= 0.0);
    
    if world.predator_count() > 0 {
        assert!(pred_energy > 0.0);
    }
    if world.prey_count() > 0 {
        assert!(prey_energy > 0.0);
    }
}

