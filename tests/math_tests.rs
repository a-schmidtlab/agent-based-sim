// Math utility tests

use predator_prey_sim::utils::math::*;

#[test]
fn test_vector_creation() {
    let v = Vector2::new(3.0, 4.0);
    assert_eq!(v.x, 3.0);
    assert_eq!(v.y, 4.0);
}

#[test]
fn test_vector_zero() {
    let v = Vector2::zero();
    assert_eq!(v.x, 0.0);
    assert_eq!(v.y, 0.0);
}

#[test]
fn test_vector_magnitude() {
    let v = Vector2::new(3.0, 4.0);
    assert_eq!(v.magnitude(), 5.0);
}

#[test]
fn test_vector_magnitude_squared() {
    let v = Vector2::new(3.0, 4.0);
    assert_eq!(v.magnitude_squared(), 25.0);
}

#[test]
fn test_vector_normalize() {
    let v = Vector2::new(3.0, 4.0);
    let normalized = v.normalize();
    assert!((normalized.magnitude() - 1.0).abs() < 1e-10);
}

#[test]
fn test_vector_scale() {
    let v = Vector2::new(3.0, 4.0);
    let scaled = v.scale(2.0);
    assert_eq!(scaled.x, 6.0);
    assert_eq!(scaled.y, 8.0);
}

#[test]
fn test_vector_limit() {
    let v = Vector2::new(3.0, 4.0); // magnitude = 5.0
    let limited = v.limit(3.0);
    assert!((limited.magnitude() - 3.0).abs() < 1e-10);
    
    let small = Vector2::new(1.0, 1.0); // magnitude ~1.41
    let limited_small = small.limit(3.0);
    assert_eq!(limited_small, small); // Should be unchanged
}

#[test]
fn test_vector_add() {
    let v1 = Vector2::new(1.0, 2.0);
    let v2 = Vector2::new(3.0, 4.0);
    let result = v1.add(&v2);
    assert_eq!(result.x, 4.0);
    assert_eq!(result.y, 6.0);
}

#[test]
fn test_vector_subtract() {
    let v1 = Vector2::new(3.0, 4.0);
    let v2 = Vector2::new(1.0, 2.0);
    let result = v1.subtract(&v2);
    assert_eq!(result.x, 2.0);
    assert_eq!(result.y, 2.0);
}

#[test]
fn test_vector_operators() {
    let v1 = Vector2::new(1.0, 2.0);
    let v2 = Vector2::new(3.0, 4.0);
    
    let sum = v1 + v2;
    assert_eq!(sum.x, 4.0);
    assert_eq!(sum.y, 6.0);
    
    let diff = v2 - v1;
    assert_eq!(diff.x, 2.0);
    assert_eq!(diff.y, 2.0);
    
    let scaled = v1 * 2.0;
    assert_eq!(scaled.x, 2.0);
    assert_eq!(scaled.y, 4.0);
}

#[test]
fn test_distance() {
    let p1 = Vector2::new(0.0, 0.0);
    let p2 = Vector2::new(3.0, 4.0);
    assert_eq!(distance(&p1, &p2), 5.0);
}

#[test]
fn test_distance_squared() {
    let p1 = Vector2::new(0.0, 0.0);
    let p2 = Vector2::new(3.0, 4.0);
    assert_eq!(distance_squared(&p1, &p2), 25.0);
}

#[test]
fn test_angle() {
    let v = Vector2::new(1.0, 0.0); // Right
    assert!((angle(&v) - 0.0).abs() < 1e-10);
    
    let v = Vector2::new(0.0, 1.0); // Up
    assert!((angle(&v) - std::f64::consts::PI / 2.0).abs() < 1e-10);
}

#[test]
fn test_from_angle() {
    let v = from_angle(0.0, 5.0);
    assert!((v.x - 5.0).abs() < 1e-10);
    assert!((v.y - 0.0).abs() < 1e-10);
    
    let v = from_angle(std::f64::consts::PI / 2.0, 5.0);
    assert!((v.x - 0.0).abs() < 1e-10);
    assert!((v.y - 5.0).abs() < 1e-10);
}

#[test]
fn test_wrap_position() {
    let pos = Vector2::new(-10.0, 50.0);
    let wrapped = wrap_position(pos, 100.0, 100.0);
    assert_eq!(wrapped.x, 90.0);
    assert_eq!(wrapped.y, 50.0);
    
    let pos = Vector2::new(110.0, 150.0);
    let wrapped = wrap_position(pos, 100.0, 100.0);
    assert_eq!(wrapped.x, 10.0);
    assert_eq!(wrapped.y, 50.0);
}

#[test]
fn test_clamp_position() {
    let pos = Vector2::new(-10.0, 50.0);
    let clamped = clamp_position(pos, 100.0, 100.0);
    assert_eq!(clamped.x, 0.0);
    assert_eq!(clamped.y, 50.0);
    
    let pos = Vector2::new(110.0, 150.0);
    let clamped = clamp_position(pos, 100.0, 100.0);
    assert_eq!(clamped.x, 100.0);
    assert_eq!(clamped.y, 100.0);
}

