use core::ops::{Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    // GLSL-like constructors
    pub fn from_scalar(s: f32) -> Self {
        Self { x: s, y: s }
    }

    // GLSL-like operations
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 0.0 {
            *self / len
        } else {
            *self
        }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    // Component-wise operations
    pub fn min(&self, other: &Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    pub fn max(&self, other: &Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    // GLSL-like swizzling (limited to basic cases)
    pub fn xx(&self) -> Self {
        Self { x: self.x, y: self.x }
    }

    pub fn yy(&self) -> Self {
        Self { x: self.y, y: self.y }
    }

    pub fn yx(&self) -> Self {
        Self { x: self.y, y: self.x }
    }
}

// Operator overloading
impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

// Assignment operators
impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

// Component-wise multiplication
impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

// Component-wise division
impl Div for Vec2 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2_construction() {
        let v = Vec2::new(1.0, 2.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);

        let v = Vec2::from_scalar(3.0);
        assert_eq!(v.x, 3.0);
        assert_eq!(v.y, 3.0);
    }

    #[test]
    fn test_vec2_operations() {
        let v1 = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(3.0, 4.0);

        // Test length
        assert_eq!(v1.length(), (1.0f32 + 4.0f32).sqrt());

        // Test normalize
        let normalized = v1.normalize();
        assert!((normalized.length() - 1.0).abs() < 1e-6);

        // Test dot product
        assert_eq!(v1.dot(&v2), 1.0 * 3.0 + 2.0 * 4.0);

        // Test min/max
        let min = v1.min(&v2);
        assert_eq!(min.x, 1.0);
        assert_eq!(min.y, 2.0);

        let max = v1.max(&v2);
        assert_eq!(max.x, 3.0);
        assert_eq!(max.y, 4.0);
    }

    #[test]
    fn test_vec2_swizzling() {
        let v = Vec2::new(1.0, 2.0);

        let xx = v.xx();
        assert_eq!(xx.x, 1.0);
        assert_eq!(xx.y, 1.0);

        let yy = v.yy();
        assert_eq!(yy.x, 2.0);
        assert_eq!(yy.y, 2.0);

        let yx = v.yx();
        assert_eq!(yx.x, 2.0);
        assert_eq!(yx.y, 1.0);
    }

    #[test]
    fn test_vec2_operators() {
        let v1 = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(3.0, 4.0);

        // Test addition
        let sum = v1 + v2;
        assert_eq!(sum.x, 4.0);
        assert_eq!(sum.y, 6.0);

        // Test subtraction
        let diff = v2 - v1;
        assert_eq!(diff.x, 2.0);
        assert_eq!(diff.y, 2.0);

        // Test scalar multiplication
        let scaled = v1 * 2.0;
        assert_eq!(scaled.x, 2.0);
        assert_eq!(scaled.y, 4.0);

        // Test scalar division
        let divided = v2 / 2.0;
        assert_eq!(divided.x, 1.5);
        assert_eq!(divided.y, 2.0);

        // Test negation
        let neg = -v1;
        assert_eq!(neg.x, -1.0);
        assert_eq!(neg.y, -2.0);

        // Test component-wise multiplication
        let mul = v1 * v2;
        assert_eq!(mul.x, 3.0);
        assert_eq!(mul.y, 8.0);

        // Test component-wise division
        let div = v2 / v1;
        assert_eq!(div.x, 3.0);
        assert_eq!(div.y, 2.0);
    }

    #[test]
    fn test_vec2_assign_operators() {
        let mut v1 = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(3.0, 4.0);

        // Test +=
        v1 += v2;
        assert_eq!(v1.x, 4.0);
        assert_eq!(v1.y, 6.0);

        // Test -=
        v1 -= v2;
        assert_eq!(v1.x, 1.0);
        assert_eq!(v1.y, 2.0);

        // Test *=
        v1 *= 2.0;
        assert_eq!(v1.x, 2.0);
        assert_eq!(v1.y, 4.0);

        // Test /=
        v1 /= 2.0;
        assert_eq!(v1.x, 1.0);
        assert_eq!(v1.y, 2.0);
    }

    #[test]
    fn test_vec2_edge_cases() {
        // Test zero vector
        let zero = Vec2::new(0.0, 0.0);
        assert_eq!(zero.length(), 0.0);
        assert_eq!(zero.normalize(), zero); // Should return self for zero vector

        // Test very small values
        let small = Vec2::new(1e-10, 1e-10);
        assert!(small.length() > 0.0);

        // Test very large values
        let large = Vec2::new(1e10, 1e10);
        assert!(large.length().is_finite());
    }
} 