use core::ops::{Add, Div, Mul, Neg, Sub};

/// 3x3 matrix for dec32-point math (GLSL-compatible, column-major storage)
///
/// Storage layout (column-major):
/// [m00, m10, m20, m01, m11, m21, m02, m12, m22]
/// Where m[row][col] represents the element at row `row` and column `col`
use super::conversions::ToDec32;
use super::dec32::Dec32;
use super::vec3::Vec3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mat3 {
    // Column-major storage: [col0, col1, col2] where each column is [x, y, z]
    // Storage: [m00, m10, m20, m01, m11, m21, m02, m12, m22]
    pub m: [Dec32; 9],
}

impl Mat3 {
    /// Create a new matrix from 9 Dec32 values (column-major order)
    ///
    /// Parameters are in column-major order:
    /// m00, m10, m20, m01, m11, m21, m02, m12, m22
    #[allow(clippy::too_many_arguments)]
    #[inline(always)]
    pub const fn new(
        m00: Dec32,
        m10: Dec32,
        m20: Dec32,
        m01: Dec32,
        m11: Dec32,
        m21: Dec32,
        m02: Dec32,
        m12: Dec32,
        m22: Dec32,
    ) -> Self {
        Mat3 {
            m: [m00, m10, m20, m01, m11, m21, m02, m12, m22],
        }
    }

    /// Create a matrix from 9 f32 values (column-major order)
    #[allow(clippy::too_many_arguments)]
    #[inline(always)]
    pub fn from_f32(
        m00: f32,
        m10: f32,
        m20: f32,
        m01: f32,
        m11: f32,
        m21: f32,
        m02: f32,
        m12: f32,
        m22: f32,
    ) -> Self {
        Mat3::new(
            m00.to_dec32(),
            m10.to_dec32(),
            m20.to_dec32(),
            m01.to_dec32(),
            m11.to_dec32(),
            m21.to_dec32(),
            m02.to_dec32(),
            m12.to_dec32(),
            m22.to_dec32(),
        )
    }

    /// Create a matrix from 3 Vec3 columns
    #[inline(always)]
    pub fn from_vec3(col0: Vec3, col1: Vec3, col2: Vec3) -> Self {
        Mat3::new(
            col0.x, col0.y, col0.z, col1.x, col1.y, col1.z, col2.x, col2.y, col2.z,
        )
    }

    /// Create identity matrix
    #[inline(always)]
    pub const fn identity() -> Self {
        Mat3::new(
            Dec32::ONE,
            Dec32(0),
            Dec32(0),
            Dec32(0),
            Dec32::ONE,
            Dec32(0),
            Dec32(0),
            Dec32(0),
            Dec32::ONE,
        )
    }

    /// Create zero matrix
    #[inline(always)]
    pub const fn zero() -> Self {
        Mat3::new(
            Dec32(0),
            Dec32(0),
            Dec32(0),
            Dec32(0),
            Dec32(0),
            Dec32(0),
            Dec32(0),
            Dec32(0),
            Dec32(0),
        )
    }

    /// Get element at row `row` and column `col`
    #[inline(always)]
    pub fn get(self, row: usize, col: usize) -> Dec32 {
        self.m[col * 3 + row]
    }

    /// Set element at row `row` and column `col`
    #[inline(always)]
    pub fn set(&mut self, row: usize, col: usize, value: Dec32) {
        self.m[col * 3 + row] = value;
    }

    /// Get column 0 as Vec3
    #[inline(always)]
    pub fn col0(self) -> Vec3 {
        Vec3::new(self.m[0], self.m[1], self.m[2])
    }

    /// Get column 1 as Vec3
    #[inline(always)]
    pub fn col1(self) -> Vec3 {
        Vec3::new(self.m[3], self.m[4], self.m[5])
    }

    /// Get column 2 as Vec3
    #[inline(always)]
    pub fn col2(self) -> Vec3 {
        Vec3::new(self.m[6], self.m[7], self.m[8])
    }

    /// Matrix-matrix multiplication
    #[allow(clippy::should_implement_trait)]
    #[inline(always)]
    pub fn mul(self, rhs: Self) -> Self {
        let a = self;
        let b = rhs;
        Mat3::new(
            // Row 0
            a.m[0] * b.m[0] + a.m[3] * b.m[1] + a.m[6] * b.m[2],
            a.m[1] * b.m[0] + a.m[4] * b.m[1] + a.m[7] * b.m[2],
            a.m[2] * b.m[0] + a.m[5] * b.m[1] + a.m[8] * b.m[2],
            // Row 1
            a.m[0] * b.m[3] + a.m[3] * b.m[4] + a.m[6] * b.m[5],
            a.m[1] * b.m[3] + a.m[4] * b.m[4] + a.m[7] * b.m[5],
            a.m[2] * b.m[3] + a.m[5] * b.m[4] + a.m[8] * b.m[5],
            // Row 2
            a.m[0] * b.m[6] + a.m[3] * b.m[7] + a.m[6] * b.m[8],
            a.m[1] * b.m[6] + a.m[4] * b.m[7] + a.m[7] * b.m[8],
            a.m[2] * b.m[6] + a.m[5] * b.m[7] + a.m[8] * b.m[8],
        )
    }

    /// Matrix-vector multiplication (mat3 * vec3)
    #[inline(always)]
    pub fn mul_vec3(self, v: Vec3) -> Vec3 {
        Vec3::new(
            self.m[0] * v.x + self.m[3] * v.y + self.m[6] * v.z,
            self.m[1] * v.x + self.m[4] * v.y + self.m[7] * v.z,
            self.m[2] * v.x + self.m[5] * v.y + self.m[8] * v.z,
        )
    }

    /// Transpose matrix
    #[inline(always)]
    pub fn transpose(self) -> Self {
        Mat3::new(
            self.m[0], self.m[3], self.m[6], self.m[1], self.m[4], self.m[7], self.m[2], self.m[5],
            self.m[8],
        )
    }

    /// Calculate determinant
    #[inline(always)]
    pub fn determinant(self) -> Dec32 {
        let m = &self.m;
        // Using Sarrus' rule for 3x3 determinant
        let a = m[0] * m[4] * m[8];
        let b = m[1] * m[5] * m[6];
        let c = m[2] * m[3] * m[7];
        let d = m[2] * m[4] * m[6];
        let e = m[0] * m[5] * m[7];
        let f = m[1] * m[3] * m[8];
        (a + b + c) - (d + e + f)
    }

    /// Calculate inverse matrix
    ///
    /// Returns None if matrix is singular (determinant is zero)
    #[inline(always)]
    pub fn inverse(self) -> Option<Self> {
        let det = self.determinant();
        if det.0 == 0 {
            return None;
        }

        let m = &self.m;
        // Calculate cofactor matrix (transposed for adjugate)
        let c00 = m[4] * m[8] - m[5] * m[7];
        let c01 = -(m[1] * m[8] - m[2] * m[7]);
        let c02 = m[1] * m[5] - m[2] * m[4];
        let c10 = -(m[3] * m[8] - m[5] * m[6]);
        let c11 = m[0] * m[8] - m[2] * m[6];
        let c12 = -(m[0] * m[5] - m[2] * m[3]);
        let c20 = m[3] * m[7] - m[4] * m[6];
        let c21 = -(m[0] * m[7] - m[1] * m[6]);
        let c22 = m[0] * m[4] - m[1] * m[3];

        // Adjugate matrix (transpose of cofactor) divided by determinant
        let inv_det = Dec32::ONE / det;
        Some(Mat3::new(
            c00 * inv_det,
            c01 * inv_det,
            c02 * inv_det,
            c10 * inv_det,
            c11 * inv_det,
            c12 * inv_det,
            c20 * inv_det,
            c21 * inv_det,
            c22 * inv_det,
        ))
    }
}

// Matrix + Matrix
impl Add for Mat3 {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Mat3::new(
            self.m[0] + rhs.m[0],
            self.m[1] + rhs.m[1],
            self.m[2] + rhs.m[2],
            self.m[3] + rhs.m[3],
            self.m[4] + rhs.m[4],
            self.m[5] + rhs.m[5],
            self.m[6] + rhs.m[6],
            self.m[7] + rhs.m[7],
            self.m[8] + rhs.m[8],
        )
    }
}

// Matrix - Matrix
impl Sub for Mat3 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Mat3::new(
            self.m[0] - rhs.m[0],
            self.m[1] - rhs.m[1],
            self.m[2] - rhs.m[2],
            self.m[3] - rhs.m[3],
            self.m[4] - rhs.m[4],
            self.m[5] - rhs.m[5],
            self.m[6] - rhs.m[6],
            self.m[7] - rhs.m[7],
            self.m[8] - rhs.m[8],
        )
    }
}

// Matrix * Matrix (matrix multiplication)
impl Mul for Mat3 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        self.mul(rhs)
    }
}

// Matrix * Vec3 (matrix-vector multiplication)
impl Mul<Vec3> for Mat3 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, rhs: Vec3) -> Vec3 {
        self.mul_vec3(rhs)
    }
}

// Matrix * Scalar
impl Mul<Dec32> for Mat3 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Dec32) -> Self {
        Mat3::new(
            self.m[0] * rhs,
            self.m[1] * rhs,
            self.m[2] * rhs,
            self.m[3] * rhs,
            self.m[4] * rhs,
            self.m[5] * rhs,
            self.m[6] * rhs,
            self.m[7] * rhs,
            self.m[8] * rhs,
        )
    }
}

// Matrix / Scalar
impl Div<Dec32> for Mat3 {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Dec32) -> Self {
        Mat3::new(
            self.m[0] / rhs,
            self.m[1] / rhs,
            self.m[2] / rhs,
            self.m[3] / rhs,
            self.m[4] / rhs,
            self.m[5] / rhs,
            self.m[6] / rhs,
            self.m[7] / rhs,
            self.m[8] / rhs,
        )
    }
}

// -Matrix
impl Neg for Mat3 {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self {
        Mat3::new(
            -self.m[0], -self.m[1], -self.m[2], -self.m[3], -self.m[4], -self.m[5], -self.m[6],
            -self.m[7], -self.m[8],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let m = Mat3::identity();
        assert_eq!(m.get(0, 0).to_f32(), 1.0);
        assert_eq!(m.get(1, 1).to_f32(), 1.0);
        assert_eq!(m.get(2, 2).to_f32(), 1.0);
        assert_eq!(m.get(0, 1).to_f32(), 0.0);
        assert_eq!(m.get(1, 0).to_f32(), 0.0);
    }

    #[test]
    fn test_zero() {
        let m = Mat3::zero();
        for i in 0..9 {
            assert_eq!(m.m[i].to_f32(), 0.0);
        }
    }

    #[test]
    fn test_add() {
        let a = Mat3::from_f32(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let b = Mat3::from_f32(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0);
        let c = a + b;
        assert_eq!(c.get(0, 0).to_f32(), 2.0);
        assert_eq!(c.get(2, 2).to_f32(), 10.0);
    }

    #[test]
    fn test_sub() {
        let a = Mat3::from_f32(5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0);
        let b = Mat3::from_f32(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0);
        let c = a - b;
        assert_eq!(c.get(0, 0).to_f32(), 4.0);
    }

    #[test]
    fn test_mul_matrix() {
        let a = Mat3::identity();
        let b = Mat3::identity();
        let c = a * b;
        assert_eq!(c, Mat3::identity());
    }

    #[test]
    fn test_mul_vec3() {
        let m = Mat3::identity();
        let v = Vec3::from_f32(1.0, 2.0, 3.0);
        let result = m * v;
        assert_eq!(result.x.to_f32(), 1.0);
        assert_eq!(result.y.to_f32(), 2.0);
        assert_eq!(result.z.to_f32(), 3.0);
    }

    #[test]
    fn test_mul_scalar() {
        let m = Mat3::from_f32(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let s = 2.0.to_dec32();
        let result = m * s;
        assert_eq!(result.get(0, 0).to_f32(), 2.0);
        assert_eq!(result.get(2, 2).to_f32(), 18.0);
    }

    #[test]
    fn test_transpose() {
        let m = Mat3::from_f32(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let t = m.transpose();
        assert_eq!(t.get(0, 1).to_f32(), m.get(1, 0).to_f32());
        assert_eq!(t.get(1, 0).to_f32(), m.get(0, 1).to_f32());
    }

    #[test]
    fn test_determinant() {
        let m = Mat3::identity();
        let det = m.determinant();
        assert!((det.to_f32() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_inverse() {
        let m = Mat3::identity();
        let inv = m.inverse().unwrap();
        assert_eq!(inv, Mat3::identity());
    }

    #[test]
    fn test_inverse_singular() {
        let m = Mat3::zero();
        assert_eq!(m.inverse(), None);
    }

    #[test]
    fn test_from_vec3() {
        let col0 = Vec3::from_f32(1.0, 2.0, 3.0);
        let col1 = Vec3::from_f32(4.0, 5.0, 6.0);
        let col2 = Vec3::from_f32(7.0, 8.0, 9.0);
        let m = Mat3::from_vec3(col0, col1, col2);
        assert_eq!(m.col0(), col0);
        assert_eq!(m.col1(), col1);
        assert_eq!(m.col2(), col2);
    }
}
