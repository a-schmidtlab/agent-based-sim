// Vector math, distance calculations

/// 2D vector for positions and velocities
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    /// Create a new vector
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Create a zero vector
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// Calculate the magnitude (length) of the vector
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Calculate the squared magnitude (faster, no sqrt)
    pub fn magnitude_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    /// Normalize the vector to unit length
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag > 0.0 {
            Self {
                x: self.x / mag,
                y: self.y / mag,
            }
        } else {
            Self::zero()
        }
    }

    /// Scale the vector by a scalar
    pub fn scale(&self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    /// Limit the magnitude to a maximum value
    pub fn limit(&self, max: f64) -> Self {
        let mag = self.magnitude();
        if mag > max {
            self.normalize().scale(max)
        } else {
            *self
        }
    }

    /// Add two vectors
    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    /// Subtract two vectors
    pub fn subtract(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Mul<f64> for Vector2 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        self.scale(scalar)
    }
}

/// Calculate the Euclidean distance between two points
pub fn distance(pos1: &Vector2, pos2: &Vector2) -> f64 {
    pos1.subtract(pos2).magnitude()
}

/// Calculate the squared distance (faster, no sqrt)
pub fn distance_squared(pos1: &Vector2, pos2: &Vector2) -> f64 {
    pos1.subtract(pos2).magnitude_squared()
}

/// Calculate the shortest distance between two points on a torus (wraparound)
/// This accounts for the fact that opposite edges are connected
pub fn distance_torus(pos1: &Vector2, pos2: &Vector2, width: f64, height: f64) -> f64 {
    let dx = (pos1.x - pos2.x).abs();
    let dy = (pos1.y - pos2.y).abs();
    
    // Consider both direct distance and wrapped distance for each axis
    let dx_wrapped = dx.min(width - dx);
    let dy_wrapped = dy.min(height - dy);
    
    // Use the minimum (wrapped) distance
    (dx_wrapped * dx_wrapped + dy_wrapped * dy_wrapped).sqrt()
}

/// Calculate the squared distance on a torus (faster, no sqrt)
pub fn distance_torus_squared(pos1: &Vector2, pos2: &Vector2, width: f64, height: f64) -> f64 {
    let dx = (pos1.x - pos2.x).abs();
    let dy = (pos1.y - pos2.y).abs();
    
    let dx_wrapped = dx.min(width - dx);
    let dy_wrapped = dy.min(height - dy);
    
    dx_wrapped * dx_wrapped + dy_wrapped * dy_wrapped
}

/// Calculate the angle of a vector in radians
pub fn angle(vector: &Vector2) -> f64 {
    vector.y.atan2(vector.x)
}

/// Create a vector from angle and magnitude
pub fn from_angle(angle: f64, magnitude: f64) -> Vector2 {
    Vector2 {
        x: angle.cos() * magnitude,
        y: angle.sin() * magnitude,
    }
}

/// Wrap a position within world bounds (toroidal)
pub fn wrap_position(pos: Vector2, width: f64, height: f64) -> Vector2 {
    let mut x = pos.x;
    let mut y = pos.y;

    if x < 0.0 {
        x += width;
    } else if x >= width {
        x -= width;
    }

    if y < 0.0 {
        y += height;
    } else if y >= height {
        y -= height;
    }

    Vector2 { x, y }
}

/// Clamp a position within world bounds (walls)
pub fn clamp_position(pos: Vector2, width: f64, height: f64) -> Vector2 {
    Vector2 {
        x: pos.x.max(0.0).min(width),
        y: pos.y.max(0.0).min(height),
    }
}
