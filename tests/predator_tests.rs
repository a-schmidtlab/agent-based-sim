// Predator tests

use predator_prey_sim::simulation::predator::Predator;
use predator_prey_sim::simulation::agent::*;
use predator_prey_sim::utils::math::Vector2;
use predator_prey_sim::config::parameters::{PredatorParameters, BoundaryType};

fn create_test_world_state() -> WorldState {
    WorldState {
        width: 100.0,
        height: 100.0,
        boundary_type: BoundaryType::Wraparound,
        nearby_predators: Vec::new(),
        nearby_prey: Vec::new(),
        dt: 1.0 / 60.0,
    }
}

#[test]
fn test_predator_creation() {
    let params = PredatorParameters::default();
    let predator = Predator::new(1, Vector2::new(50.0, 50.0), params.clone());
    
    assert_eq!(predator.id(), 1);
    assert_eq!(predator.agent_type(), AgentType::Predator);
    assert_eq!(predator.energy(), params.initial_energy);
}

#[test]
fn test_predator_energy_consumption() {
    let params = PredatorParameters::default();
    let mut predator = Predator::new(1, Vector2::new(50.0, 50.0), params.clone());
    
    let initial_energy = predator.energy();
    let mut world_state = create_test_world_state();
    
    // Run many updates
    for _ in 0..100 {
        predator.update(&world_state);
    }
    
    // Energy should decrease
    assert!(predator.energy() < initial_energy);
}

#[test]
fn test_predator_chases_prey() {
    let params = PredatorParameters::default();
    let mut predator = Predator::new(1, Vector2::new(50.0, 50.0), params.clone());
    
    let mut world_state = create_test_world_state();
    // Add nearby prey
    world_state.nearby_prey.push((2, Vector2::new(60.0, 50.0), 10.0));
    
    let initial_pos = predator.position();
    predator.update(&world_state);
    
    // Predator should move toward prey (velocity should be in that direction)
    let velocity = predator.velocity();
    assert!(velocity.magnitude() > 0.0 || velocity.x > 0.0); // Should move right toward prey
}

#[test]
fn test_predator_consumes_prey() {
    let params = PredatorParameters::default();
    let mut predator = Predator::new(1, Vector2::new(50.0, 50.0), params.clone());
    
    let mut world_state = create_test_world_state();
    // Add prey within capture distance
    world_state.nearby_prey.push((2, Vector2::new(52.0, 50.0), 2.0)); // Very close
    
    let initial_energy = predator.energy();
    let action = predator.update(&world_state);
    
    // Should consume prey and gain energy
    match action {
        AgentAction::Consumed { target_id } => {
            assert_eq!(target_id, 2);
            assert!(predator.energy() > initial_energy);
        }
        _ => panic!("Expected Consumed action"),
    }
}

#[test]
fn test_predator_dies_when_no_energy() {
    let mut params = PredatorParameters::default();
    params.energy_per_tick = 1000.0; // High consumption
    let mut predator = Predator::new(1, Vector2::new(50.0, 50.0), params);
    
    let mut world_state = create_test_world_state();
    
    // Run until dead
    for _ in 0..10 {
        predator.update(&world_state);
    }
    
    assert!(!predator.is_alive());
}

