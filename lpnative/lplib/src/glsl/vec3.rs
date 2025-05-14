use core::ops::{Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    // GLSL-like constructors
    pub fn from_scalar(s: f32) -> Self {
        Self { x: s, y: s, z: s }
    }

    // GLSL-like operations
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
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
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    // Component-wise operations
    pub fn min(&self, other: &Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }

    pub fn max(&self, other: &Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }

    // GLSL-like swizzling (limited to basic cases)
    pub fn xxx(&self) -> Self {
        Self { x: self.x, y: self.x, z: self.x }
    }

    pub fn yyy(&self) -> Self {
        Self { x: self.y, y: self.y, z: self.y }
    }

    pub fn zzz(&self) -> Self {
        Self { x: self.z, y: self.z, z: self.z }
    }

    pub fn xyz(&self) -> Self {
        *self
    }

    pub fn zyx(&self) -> Self {
        Self { x: self.z, y: self.y, z: self.x }
    }
}

// Operator overloading
impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// Assignment operators
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

// Component-wise multiplication
impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

// Component-wise division
impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_construction() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);

        let v = Vec3::from_scalar(3.0);
        assert_eq!(v.x, 3.0);
        assert_eq!(v.y, 3.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_vec3_operations() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        // Test length
        assert_eq!(v1.length(), (1.0f32 + 4.0f32 + 9.0f32).sqrt());

        // Test normalize
        let normalized = v1.normalize();
        assert!((normalized.length() - 1.0).abs() < 1e-6);

        // Test dot product
        assert_eq!(v1.dot(&v2), 1.0 * 4.0 + 2.0 * 5.0 + 3.0 * 6.0);

        // Test cross product
        let cross = v1.cross(&v2);
        assert_eq!(cross.x, 2.0 * 6.0 - 3.0 * 5.0);
        assert_eq!(cross.y, 3.0 * 4.0 - 1.0 * 6.0);
        assert_eq!(cross.z, 1.0 * 5.0 - 2.0 * 4.0);

        // Test min/max
        let min = v1.min(&v2);
        assert_eq!(min.x, 1.0);
        assert_eq!(min.y, 2.0);
        assert_eq!(min.z, 3.0);

        let max = v1.max(&v2);
        assert_eq!(max.x, 4.0);
        assert_eq!(max.y, 5.0);
        assert_eq!(max.z, 6.0);
    }

    #[test]
    fn test_vec3_swizzling() {
        let v = Vec3::new(1.0, 2.0, 3.0);

        let xxx = v.xxx();
        assert_eq!(xxx.x, 1.0);
        assert_eq!(xxx.y, 1.0);
        assert_eq!(xxx.z, 1.0);

        let zyx = v.zyx();
        assert_eq!(zyx.x, 3.0);
        assert_eq!(zyx.y, 2.0);
        assert_eq!(zyx.z, 1.0);
    }

    #[test]
    fn test_vec3_operators() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        // Test addition
        let sum = v1 + v2;
        assert_eq!(sum.x, 5.0);
        assert_eq!(sum.y, 7.0);
        assert_eq!(sum.z, 9.0);

        // Test subtraction
        let diff = v2 - v1;
        assert_eq!(diff.x, 3.0);
        assert_eq!(diff.y, 3.0);
        assert_eq!(diff.z, 3.0);

        // Test scalar multiplication
        let scaled = v1 * 2.0;
        assert_eq!(scaled.x, 2.0);
        assert_eq!(scaled.y, 4.0);
        assert_eq!(scaled.z, 6.0);

        // Test scalar division
        let divided = v2 / 2.0;
        assert_eq!(divided.x, 2.0);
        assert_eq!(divided.y, 2.5);
        assert_eq!(divided.z, 3.0);

        // Test negation
        let neg = -v1;
        assert_eq!(neg.x, -1.0);
        assert_eq!(neg.y, -2.0);
        assert_eq!(neg.z, -3.0);

        // Test component-wise multiplication
        let mul = v1 * v2;
        assert_eq!(mul.x, 4.0);
        assert_eq!(mul.y, 10.0);
        assert_eq!(mul.z, 18.0);

        // Test component-wise division
        let div = v2 / v1;
        assert_eq!(div.x, 4.0);
        assert_eq!(div.y, 2.5);
        assert_eq!(div.z, 2.0);
    }

    #[test]
    fn test_vec3_assign_operators() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        // Test +=
        v1 += v2;
        assert_eq!(v1.x, 5.0);
        assert_eq!(v1.y, 7.0);
        assert_eq!(v1.z, 9.0);

        // Test -=
        v1 -= v2;
        assert_eq!(v1.x, 1.0);
        assert_eq!(v1.y, 2.0);
        assert_eq!(v1.z, 3.0);

        // Test *=
        v1 *= 2.0;
        assert_eq!(v1.x, 2.0);
        assert_eq!(v1.y, 4.0);
        assert_eq!(v1.z, 6.0);

        // Test /=
        v1 /= 2.0;
        assert_eq!(v1.x, 1.0);
        assert_eq!(v1.y, 2.0);
        assert_eq!(v1.z, 3.0);
    }

    #[test]
    fn test_vec3_edge_cases() {
        // Test zero vector
        let zero = Vec3::new(0.0, 0.0, 0.0);
        assert_eq!(zero.length(), 0.0);
        assert_eq!(zero.normalize(), zero); // Should return self for zero vector

        // Test very small values
        let small = Vec3::new(1e-10, 1e-10, 1e-10);
        assert!(small.length() > 0.0);

        // Test very large values
        let large = Vec3::new(1e10, 1e10, 1e10);
        assert!(large.length().is_finite());
    }
} 