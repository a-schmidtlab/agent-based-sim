// Parameters tests

use predator_prey_sim::config::parameters::*;

#[test]
fn test_predator_parameters_default() {
    let params = PredatorParameters::default();
    assert!(params.initial_energy > 0.0);
    assert!(params.max_speed > 0.0);
    assert!(params.perception_radius > 0.0);
    assert!(params.capture_distance > 0.0);
}

#[test]
fn test_prey_parameters_default() {
    let params = PreyParameters::default();
    assert!(params.initial_energy > 0.0);
    assert!(params.max_speed > 0.0);
    assert!(params.detection_radius > 0.0);
}

#[test]
fn test_world_parameters_default() {
    let params = WorldParameters::default();
    assert!(params.width > 0.0);
    assert!(params.height > 0.0);
}

#[test]
fn test_simulation_parameters_default() {
    let params = SimulationParameters::default();
    assert!(params.tick_rate > 0.0);
    assert!(params.max_agents > 0);
}

#[test]
fn test_simulation_parameters_update_dt() {
    let mut params = SimulationParameters::default();
    params.tick_rate = 30.0;
    params.update_dt();
    assert_eq!(params.dt, 1.0 / 30.0);
}

#[test]
fn test_parameters_default() {
    let params = Parameters::default();
    assert!(params.validate().is_ok());
}

#[test]
fn test_parameters_validate_success() {
    let params = Parameters::default();
    assert!(params.validate().is_ok());
}

#[test]
fn test_parameters_validate_failure_negative_width() {
    let mut params = Parameters::default();
    params.world.width = -10.0;
    assert!(params.validate().is_err());
}

#[test]
fn test_parameters_validate_failure_zero_speed() {
    let mut params = Parameters::default();
    params.predator.max_speed = 0.0;
    assert!(params.validate().is_err());
}

#[test]
fn test_parameters_validate_failure_zero_energy() {
    let mut params = Parameters::default();
    params.prey.initial_energy = 0.0;
    assert!(params.validate().is_err());
}

