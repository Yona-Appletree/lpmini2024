use core::ops::{Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    // GLSL-like constructors
    pub fn from_scalar(s: f32) -> Self {
        Self { x: s, y: s, z: s, w: s }
    }

    // GLSL-like operations
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
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
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    // Component-wise operations
    pub fn min(&self, other: &Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
            w: self.w.min(other.w),
        }
    }

    pub fn max(&self, other: &Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
            w: self.w.max(other.w),
        }
    }

    // GLSL-like swizzling (limited to basic cases)
    pub fn xxxx(&self) -> Self {
        Self { x: self.x, y: self.x, z: self.x, w: self.x }
    }

    pub fn yyyy(&self) -> Self {
        Self { x: self.y, y: self.y, z: self.y, w: self.y }
    }

    pub fn zzzz(&self) -> Self {
        Self { x: self.z, y: self.z, z: self.z, w: self.z }
    }

    pub fn wwww(&self) -> Self {
        Self { x: self.w, y: self.w, z: self.w, w: self.w }
    }

    pub fn xyzw(&self) -> Self {
        *self
    }

    pub fn wzyx(&self) -> Self {
        Self { x: self.w, y: self.z, z: self.y, w: self.x }
    }
}

// Operator overloading
impl Add for Vec4 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Vec4 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }
}

impl Div<f32> for Vec4 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w / scalar,
        }
    }
}

impl Neg for Vec4 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

// Assignment operators
impl AddAssign for Vec4 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
    }
}

impl SubAssign for Vec4 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.w -= other.w;
    }
}

impl MulAssign<f32> for Vec4 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
        self.w *= scalar;
    }
}

impl DivAssign<f32> for Vec4 {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
        self.w /= scalar;
    }
}

// Component-wise multiplication
impl Mul for Vec4 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        }
    }
}

// Component-wise division
impl Div for Vec4 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            w: self.w / other.w,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec4_construction() {
        let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
        assert_eq!(v.w, 4.0);

        let v = Vec4::from_scalar(3.0);
        assert_eq!(v.x, 3.0);
        assert_eq!(v.y, 3.0);
        assert_eq!(v.z, 3.0);
        assert_eq!(v.w, 3.0);
    }

    #[test]
    fn test_vec4_operations() {
        let v1 = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vec4::new(5.0, 6.0, 7.0, 8.0);

        // Test length
        assert_eq!(v1.length(), (1.0f32 + 4.0f32 + 9.0f32 + 16.0f32).sqrt());

        // Test normalize
        let normalized = v1.normalize();
        assert!((normalized.length() - 1.0).abs() < 1e-6);

        // Test dot product
        assert_eq!(v1.dot(&v2), 1.0 * 5.0 + 2.0 * 6.0 + 3.0 * 7.0 + 4.0 * 8.0);

        // Test min/max
        let min = v1.min(&v2);
        assert_eq!(min.x, 1.0);
        assert_eq!(min.y, 2.0);
        assert_eq!(min.z, 3.0);
        assert_eq!(min.w, 4.0);

        let max = v1.max(&v2);
        assert_eq!(max.x, 5.0);
        assert_eq!(max.y, 6.0);
        assert_eq!(max.z, 7.0);
        assert_eq!(max.w, 8.0);
    }

    #[test]
    fn test_vec4_swizzling() {
        let v = Vec4::new(1.0, 2.0, 3.0, 4.0);

        let xxxx = v.xxxx();
        assert_eq!(xxxx.x, 1.0);
        assert_eq!(xxxx.y, 1.0);
        assert_eq!(xxxx.z, 1.0);
        assert_eq!(xxxx.w, 1.0);

        let wzyx = v.wzyx();
        assert_eq!(wzyx.x, 4.0);
        assert_eq!(wzyx.y, 3.0);
        assert_eq!(wzyx.z, 2.0);
        assert_eq!(wzyx.w, 1.0);
    }

    #[test]
    fn test_vec4_operators() {
        let v1 = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vec4::new(5.0, 6.0, 7.0, 8.0);

        // Test addition
        let sum = v1 + v2;
        assert_eq!(sum.x, 6.0);
        assert_eq!(sum.y, 8.0);
        assert_eq!(sum.z, 10.0);
        assert_eq!(sum.w, 12.0);

        // Test subtraction
        let diff = v2 - v1;
        assert_eq!(diff.x, 4.0);
        assert_eq!(diff.y, 4.0);
        assert_eq!(diff.z, 4.0);
        assert_eq!(diff.w, 4.0);

        // Test scalar multiplication
        let scaled = v1 * 2.0;
        assert_eq!(scaled.x, 2.0);
        assert_eq!(scaled.y, 4.0);
        assert_eq!(scaled.z, 6.0);
        assert_eq!(scaled.w, 8.0);

        // Test scalar division
        let divided = v2 / 2.0;
        assert_eq!(divided.x, 2.5);
        assert_eq!(divided.y, 3.0);
        assert_eq!(divided.z, 3.5);
        assert_eq!(divided.w, 4.0);

        // Test negation
        let neg = -v1;
        assert_eq!(neg.x, -1.0);
        assert_eq!(neg.y, -2.0);
        assert_eq!(neg.z, -3.0);
        assert_eq!(neg.w, -4.0);

        // Test component-wise multiplication
        let mul = v1 * v2;
        assert_eq!(mul.x, 5.0);
        assert_eq!(mul.y, 12.0);
        assert_eq!(mul.z, 21.0);
        assert_eq!(mul.w, 32.0);

        // Test component-wise division
        let div = v2 / v1;
        assert_eq!(div.x, 5.0);
        assert_eq!(div.y, 3.0);
        assert_eq!(div.z, 7.0/3.0);
        assert_eq!(div.w, 2.0);
    }

    #[test]
    fn test_vec4_assign_operators() {
        let mut v1 = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vec4::new(5.0, 6.0, 7.0, 8.0);

        // Test +=
        v1 += v2;
        assert_eq!(v1.x, 6.0);
        assert_eq!(v1.y, 8.0);
        assert_eq!(v1.z, 10.0);
        assert_eq!(v1.w, 12.0);

        // Test -=
        v1 -= v2;
        assert_eq!(v1.x, 1.0);
        assert_eq!(v1.y, 2.0);
        assert_eq!(v1.z, 3.0);
        assert_eq!(v1.w, 4.0);

        // Test *=
        v1 *= 2.0;
        assert_eq!(v1.x, 2.0);
        assert_eq!(v1.y, 4.0);
        assert_eq!(v1.z, 6.0);
        assert_eq!(v1.w, 8.0);

        // Test /=
        v1 /= 2.0;
        assert_eq!(v1.x, 1.0);
        assert_eq!(v1.y, 2.0);
        assert_eq!(v1.z, 3.0);
        assert_eq!(v1.w, 4.0);
    }

    #[test]
    fn test_vec4_edge_cases() {
        // Test zero vector
        let zero = Vec4::new(0.0, 0.0, 0.0, 0.0);
        assert_eq!(zero.length(), 0.0);
        assert_eq!(zero.normalize(), zero); // Should return self for zero vector

        // Test very small values
        let small = Vec4::new(1e-10, 1e-10, 1e-10, 1e-10);
        assert!(small.length() > 0.0);

        // Test very large values
        let large = Vec4::new(1e10, 1e10, 1e10, 1e10);
        assert!(large.length().is_finite());
    }
} 