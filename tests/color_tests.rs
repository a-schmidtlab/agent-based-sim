// Color utility tests

use predator_prey_sim::utils::color::*;

#[test]
fn test_color_rgb() {
    let color = Color::rgb(255, 128, 64);
    assert_eq!(color.r, 255);
    assert_eq!(color.g, 128);
    assert_eq!(color.b, 64);
    assert_eq!(color.a, 255);
}

#[test]
fn test_color_rgba() {
    let color = Color::rgba(255, 128, 64, 200);
    assert_eq!(color.r, 255);
    assert_eq!(color.g, 128);
    assert_eq!(color.b, 64);
    assert_eq!(color.a, 200);
}

#[test]
fn test_color_lerp() {
    let c1 = Color::rgb(0, 0, 0);
    let c2 = Color::rgb(255, 255, 255);
    
    let mid = c1.lerp(&c2, 0.5);
    assert_eq!(mid.r, 127);
    assert_eq!(mid.g, 127);
    assert_eq!(mid.b, 127);
    
    let start = c1.lerp(&c2, 0.0);
    assert_eq!(start, c1);
    
    let end = c1.lerp(&c2, 1.0);
    assert_eq!(end, c2);
}

#[test]
fn test_colors_predator() {
    let color = Colors::predator();
    assert!(color.r > 200); // Should be red
}

#[test]
fn test_colors_prey() {
    let color = Colors::prey();
    assert!(color.g > 100); // Should be green
}

#[test]
fn test_energy_color() {
    let low = Colors::energy_color(0.0, true);
    let high = Colors::energy_color(1.0, true);
    assert_ne!(low, high);
    
    let mid = Colors::energy_color(0.5, true);
    // Mid should be between low and high
    assert!(mid.r >= low.r.min(high.r));
    assert!(mid.r <= low.r.max(high.r));
}

