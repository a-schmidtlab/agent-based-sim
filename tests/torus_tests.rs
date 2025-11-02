// Torus topography tests

use predator_prey_sim::utils::math::{Vector2, distance_torus, distance_torus_squared, wrap_position};

#[test]
fn test_torus_distance_same_position() {
    let p1 = Vector2::new(50.0, 50.0);
    let p2 = Vector2::new(50.0, 50.0);
    assert_eq!(distance_torus(&p1, &p2, 100.0, 100.0), 0.0);
}

#[test]
fn test_torus_distance_normal() {
    let p1 = Vector2::new(10.0, 10.0);
    let p2 = Vector2::new(20.0, 20.0);
    let dist = distance_torus(&p1, &p2, 100.0, 100.0);
    let expected = ((10.0f64).powi(2) + (10.0f64).powi(2)).sqrt();
    assert!((dist - expected).abs() < 1e-10);
}

#[test]
fn test_torus_distance_wraparound_x() {
    let p1 = Vector2::new(10.0, 50.0);
    let p2 = Vector2::new(90.0, 50.0);
    // Direct distance: 80
    // Wrapped distance: 20 (via edge)
    let dist = distance_torus(&p1, &p2, 100.0, 100.0);
    assert!((dist - 20.0).abs() < 1e-10, "Expected ~20.0, got {}", dist);
}

#[test]
fn test_torus_distance_wraparound_y() {
    let p1 = Vector2::new(50.0, 10.0);
    let p2 = Vector2::new(50.0, 90.0);
    let dist = distance_torus(&p1, &p2, 100.0, 100.0);
    assert!((dist - 20.0).abs() < 1e-10, "Expected ~20.0, got {}", dist);
}

#[test]
fn test_torus_distance_wraparound_both() {
    let p1 = Vector2::new(10.0, 10.0);
    let p2 = Vector2::new(90.0, 90.0);
    // Direct: sqrt(80² + 80²) ≈ 113.1
    // Wrapped: sqrt(20² + 20²) ≈ 28.3
    let dist = distance_torus(&p1, &p2, 100.0, 100.0);
    let expected_wrapped = ((20.0f64).powi(2) + (20.0f64).powi(2)).sqrt();
    assert!((dist - expected_wrapped).abs() < 1e-10);
}

#[test]
fn test_torus_distance_squared() {
    let p1 = Vector2::new(10.0, 50.0);
    let p2 = Vector2::new(90.0, 50.0);
    let dist_sq = distance_torus_squared(&p1, &p2, 100.0, 100.0);
    let expected = 20.0 * 20.0;
    assert!((dist_sq - expected).abs() < 1e-10);
}

#[test]
fn test_wrap_position_negative_x() {
    let pos = Vector2::new(-10.0, 50.0);
    let wrapped = wrap_position(pos, 100.0, 100.0);
    assert!((wrapped.x - 90.0).abs() < 1e-10);
    assert_eq!(wrapped.y, 50.0);
}

#[test]
fn test_wrap_position_overflow_x() {
    let pos = Vector2::new(110.0, 50.0);
    let wrapped = wrap_position(pos, 100.0, 100.0);
    assert!((wrapped.x - 10.0).abs() < 1e-10);
    assert_eq!(wrapped.y, 50.0);
}

#[test]
fn test_wrap_position_corners() {
    // Top-left corner wraps to bottom-right
    let pos = Vector2::new(-5.0, -5.0);
    let wrapped = wrap_position(pos, 100.0, 100.0);
    assert!((wrapped.x - 95.0).abs() < 1e-10);
    assert!((wrapped.y - 95.0).abs() < 1e-10);
    
    // Bottom-right corner wraps to top-left
    let pos = Vector2::new(105.0, 105.0);
    let wrapped = wrap_position(pos, 100.0, 100.0);
    assert!((wrapped.x - 5.0).abs() < 1e-10);
    assert!((wrapped.y - 5.0).abs() < 1e-10);
}

