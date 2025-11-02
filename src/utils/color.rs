// Color schemes for visualization

/// RGB color representation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    /// Create a new color with RGBA values
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Create a new color with RGB values (alpha = 255)
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Convert to egui color32 format
    pub fn to_egui_color32(&self) -> egui::Color32 {
        egui::Color32::from_rgba_unmultiplied(self.r, self.g, self.b, self.a)
    }

    /// Interpolate between two colors based on a factor (0.0 to 1.0)
    pub fn lerp(&self, other: &Self, factor: f64) -> Self {
        let factor = factor.max(0.0).min(1.0);
        Self {
            r: ((self.r as f64 * (1.0 - factor)) + (other.r as f64 * factor)) as u8,
            g: ((self.g as f64 * (1.0 - factor)) + (other.g as f64 * factor)) as u8,
            b: ((self.b as f64 * (1.0 - factor)) + (other.b as f64 * factor)) as u8,
            a: ((self.a as f64 * (1.0 - factor)) + (other.a as f64 * factor)) as u8,
        }
    }
}

/// Predefined colors for the simulation
pub struct Colors;

impl Colors {
    /// Predator color (red)
    pub fn predator() -> Color {
        Color::rgb(220, 20, 60) // Crimson red
    }

    /// Prey color (green)
    pub fn prey() -> Color {
        Color::rgb(34, 139, 34) // Forest green
    }

    /// Background color
    pub fn background() -> Color {
        Color::rgb(240, 240, 240) // Light gray
    }

    /// Get color based on energy level (for visualization)
    /// factor: 0.0 (low energy) to 1.0 (high energy)
    pub fn energy_color(factor: f64, is_predator: bool) -> Color {
        let factor = factor.max(0.0).min(1.0);
        if is_predator {
            // Predator: red when low energy, bright red when high
            let low = Color::rgb(139, 0, 0);   // Dark red
            let high = Color::rgb(255, 69, 0); // Bright orange-red
            low.lerp(&high, factor)
        } else {
            // Prey: dark green when low energy, bright green when high
            let low = Color::rgb(0, 100, 0);     // Dark green
            let high = Color::rgb(144, 238, 144); // Light green
            low.lerp(&high, factor)
        }
    }

    /// Grid line color
    pub fn grid() -> Color {
        Color::rgb(200, 200, 200)
    }

    /// Text color
    pub fn text() -> Color {
        Color::rgb(30, 30, 30)
    }
}
