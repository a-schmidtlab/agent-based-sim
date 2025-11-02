// Agent tests

use predator_prey_sim::simulation::agent::*;
use predator_prey_sim::utils::math::Vector2;
use predator_prey_sim::config::parameters::BoundaryType;

#[test]
fn test_base_agent_creation() {
    let agent = BaseAgent::new(
        1,
        AgentType::Predator,
        Vector2::new(10.0, 20.0),
        100.0,
        2.0,
    );

    assert_eq!(agent.id, 1);
    assert_eq!(agent.agent_type, AgentType::Predator);
    assert_eq!(agent.position.x, 10.0);
    assert_eq!(agent.position.y, 20.0);
    assert_eq!(agent.energy, 100.0);
    assert_eq!(agent.age, 0);
    assert_eq!(agent.max_speed, 2.0);
}

#[test]
fn test_base_agent_velocity_limit() {
    let mut agent = BaseAgent::new(
        1,
        AgentType::Predator,
        Vector2::new(0.0, 0.0),
        100.0,
        2.0,
    );

    // Try to set a velocity that exceeds max speed
    let fast_velocity = Vector2::new(10.0, 0.0);
    agent.set_velocity(fast_velocity);

    // Velocity should be limited
    assert!(agent.velocity.magnitude() <= 2.0 + 1e-10);
}

#[test]
fn test_base_agent_energy_consumption() {
    let mut agent = BaseAgent::new(
        1,
        AgentType::Predator,
        Vector2::new(0.0, 0.0),
        100.0,
        2.0,
    );

    agent.consume_energy(30.0);
    assert_eq!(agent.energy, 70.0);

    agent.consume_energy(100.0); // More than available
    assert_eq!(agent.energy, 0.0); // Should not go negative
}

#[test]
fn test_base_agent_energy_addition() {
    let mut agent = BaseAgent::new(
        1,
        AgentType::Predator,
        Vector2::new(0.0, 0.0),
        100.0,
        2.0,
    );

    agent.add_energy(50.0);
    assert_eq!(agent.energy, 150.0);
}

#[test]
fn test_base_agent_age_increment() {
    let mut agent = BaseAgent::new(
        1,
        AgentType::Predator,
        Vector2::new(0.0, 0.0),
        100.0,
        2.0,
    );

    assert_eq!(agent.age, 0);
    agent.increment_age();
    assert_eq!(agent.age, 1);
    agent.increment_age();
    assert_eq!(agent.age, 2);
}

#[test]
fn test_base_agent_alive_check() {
    let mut agent = BaseAgent::new(
        1,
        AgentType::Predator,
        Vector2::new(0.0, 0.0),
        100.0,
        2.0,
    );

    assert!(agent.check_alive());
    agent.consume_energy(50.0);
    assert!(agent.check_alive());
    agent.consume_energy(50.0);
    assert!(!agent.check_alive()); // Energy is 0
}

#[test]
fn test_base_agent_position_update_wraparound() {
    let mut agent = BaseAgent::new(
        1,
        AgentType::Predator,
        Vector2::new(50.0, 50.0),
        100.0,
        2.0,
    );

    agent.set_velocity(Vector2::new(1.0, 0.0));

    let world_state = WorldState {
        width: 100.0,
        height: 100.0,
        boundary_type: BoundaryType::Wraparound,
        nearby_predators: Vec::new(),
        nearby_prey: Vec::new(),
        dt: 60.0, // Large dt to test wrapping
    };

    agent.update_position(&world_state);
    
    // Should wrap around
    assert!(agent.position.x < 100.0);
    assert!(agent.position.y >= 0.0);
}

#[test]
fn test_base_agent_position_update_walls() {
    let mut agent = BaseAgent::new(
        1,
        AgentType::Predator,
        Vector2::new(50.0, 50.0),
        100.0,
        2.0,
    );

    agent.set_velocity(Vector2::new(10.0, 10.0)); // Fast movement

    let world_state = WorldState {
        width: 100.0,
        height: 100.0,
        boundary_type: BoundaryType::Walls,
        nearby_predators: Vec::new(),
        nearby_prey: Vec::new(),
        dt: 10.0, // Large dt
    };

    agent.update_position(&world_state);

    // Should be clamped within bounds
    assert!(agent.position.x >= 0.0 && agent.position.x <= 100.0);
    assert!(agent.position.y >= 0.0 && agent.position.y <= 100.0);
}
