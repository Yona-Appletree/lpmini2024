/// Type-safe VM stack implementation
extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

use lp_math::dec32::{Dec32, Mat3, Vec2, Vec3, Vec4};

use super::error::LpsVmError;

/// VM Stack for LPS execution
///
/// Internally stores raw i32 values for type-independence and performance,
/// but provides type-safe push/pop methods for Dec32, Int32, and vector types.
pub struct ValueStack {
    data: Vec<i32>,
    sp: usize,
    max_size: usize,
}

impl ValueStack {
    /// Create a new stack with the given maximum size
    pub fn try_new(max_size: usize) -> Result<Self, LpsVmError> {
        let data = if max_size > 0 {
            vec![0; max_size]
        } else {
            Vec::new()
        };

        Ok(ValueStack {
            data,
            sp: 0,
            max_size,
        })
    }

    /// Create a new stack and panic if allocation fails.
    pub fn new(max_size: usize) -> Self {
        Self::try_new(max_size).expect("value stack allocation failed")
    }

    /// Reset the stack pointer to 0
    #[inline(always)]
    pub fn reset(&mut self) {
        self.sp = 0;
    }

    /// Get the current stack pointer
    #[inline(always)]
    pub fn sp(&self) -> usize {
        self.sp
    }

    /// Get raw slice access (read-only)
    #[inline(always)]
    pub fn raw_slice(&self) -> &[i32] {
        self.data.as_slice()
    }

    /// Get mutable raw slice access
    #[inline(always)]
    pub fn raw_slice_mut(&mut self) -> &mut [i32] {
        self.data.as_mut_slice()
    }

    /// Get current stack contents as Vec<Dec32>
    ///
    /// Returns all values currently on the stack (0..sp) as Dec32 values.
    /// This allocates a new Vec - use sparingly.
    #[inline(always)]
    pub fn to_vec_dec32(&self) -> Vec<Dec32> {
        self.data
            .as_slice()
            .iter()
            .take(self.sp)
            .copied()
            .map(Dec32)
            .collect()
    }

    // === Basic push/pop for single values ===

    /// Push a Dec32 value onto the stack
    #[inline(always)]
    pub fn push_dec32(&mut self, val: Dec32) -> Result<(), LpsVmError> {
        if self.sp >= self.max_size {
            return Err(LpsVmError::StackOverflow { sp: self.sp });
        }
        self.data[self.sp] = val.0;
        self.sp += 1;
        Ok(())
    }

    /// Pop a Dec32 value from the stack
    #[inline(always)]
    pub fn pop_dec32(&mut self) -> Result<Dec32, LpsVmError> {
        if self.sp == 0 {
            return Err(LpsVmError::StackUnderflow {
                required: 1,
                actual: 0,
            });
        }
        self.sp -= 1;
        Ok(Dec32(self.data[self.sp]))
    }

    /// Push an Int32 value onto the stack
    #[inline(always)]
    pub fn push_int32(&mut self, val: i32) -> Result<(), LpsVmError> {
        if self.sp >= self.max_size {
            return Err(LpsVmError::StackOverflow { sp: self.sp });
        }
        self.data[self.sp] = val;
        self.sp += 1;
        Ok(())
    }

    /// Pop an Int32 value from the stack
    #[inline(always)]
    pub fn pop_int32(&mut self) -> Result<i32, LpsVmError> {
        if self.sp == 0 {
            return Err(LpsVmError::StackUnderflow {
                required: 1,
                actual: 0,
            });
        }
        self.sp -= 1;
        Ok(self.data[self.sp])
    }

    // === Multi-value operations ===

    /// Pop 2 raw i32 values from the stack (returns in stack order: bottom, top)
    #[inline(always)]
    pub fn pop2(&mut self) -> Result<(i32, i32), LpsVmError> {
        if self.sp < 2 {
            return Err(LpsVmError::StackUnderflow {
                required: 2,
                actual: self.sp,
            });
        }
        self.sp -= 1;
        let b = self.data[self.sp];
        self.sp -= 1;
        let a = self.data[self.sp];
        Ok((a, b))
    }

    /// Pop 3 raw i32 values from the stack
    #[inline(always)]
    pub fn pop3(&mut self) -> Result<(i32, i32, i32), LpsVmError> {
        if self.sp < 3 {
            return Err(LpsVmError::StackUnderflow {
                required: 3,
                actual: self.sp,
            });
        }
        self.sp -= 1;
        let c = self.data[self.sp];
        self.sp -= 1;
        let b = self.data[self.sp];
        self.sp -= 1;
        let a = self.data[self.sp];
        Ok((a, b, c))
    }

    /// Pop 4 raw i32 values from the stack
    #[inline(always)]
    pub fn pop4(&mut self) -> Result<(i32, i32, i32, i32), LpsVmError> {
        if self.sp < 4 {
            return Err(LpsVmError::StackUnderflow {
                required: 4,
                actual: self.sp,
            });
        }
        self.sp -= 1;
        let d = self.data[self.sp];
        self.sp -= 1;
        let c = self.data[self.sp];
        self.sp -= 1;
        let b = self.data[self.sp];
        self.sp -= 1;
        let a = self.data[self.sp];
        Ok((a, b, c, d))
    }

    /// Push 2 raw i32 values onto the stack
    #[inline(always)]
    pub fn push2(&mut self, v0: i32, v1: i32) -> Result<(), LpsVmError> {
        if self.sp + 2 > self.max_size {
            return Err(LpsVmError::StackOverflow { sp: self.sp });
        }
        self.data[self.sp] = v0;
        self.data[self.sp + 1] = v1;
        self.sp += 2;
        Ok(())
    }

    /// Push 3 raw i32 values onto the stack
    #[inline(always)]
    pub fn push3(&mut self, v0: i32, v1: i32, v2: i32) -> Result<(), LpsVmError> {
        if self.sp + 3 > self.max_size {
            return Err(LpsVmError::StackOverflow { sp: self.sp });
        }
        self.data[self.sp] = v0;
        self.data[self.sp + 1] = v1;
        self.data[self.sp + 2] = v2;
        self.sp += 3;
        Ok(())
    }

    /// Push 4 raw i32 values onto the stack
    #[inline(always)]
    pub fn push4(&mut self, v0: i32, v1: i32, v2: i32, v3: i32) -> Result<(), LpsVmError> {
        if self.sp + 4 > self.max_size {
            return Err(LpsVmError::StackOverflow { sp: self.sp });
        }
        self.data[self.sp] = v0;
        self.data[self.sp + 1] = v1;
        self.data[self.sp + 2] = v2;
        self.data[self.sp + 3] = v3;
        self.sp += 4;
        Ok(())
    }

    // === Vec2/3/4 operations ===

    /// Push a Vec2 onto the stack (as 2 Dec32 values)
    #[inline(always)]
    pub fn push_vec2(&mut self, v: Vec2) -> Result<(), LpsVmError> {
        self.push2(v.x.0, v.y.0)
    }

    /// Pop a Vec2 from the stack
    #[inline(always)]
    pub fn pop_vec2(&mut self) -> Result<Vec2, LpsVmError> {
        let (x, y) = self.pop2()?;
        Ok(Vec2::new(Dec32(x), Dec32(y)))
    }

    /// Push a Vec3 onto the stack (as 3 Dec32 values)
    #[inline(always)]
    pub fn push_vec3(&mut self, v: Vec3) -> Result<(), LpsVmError> {
        self.push3(v.x.0, v.y.0, v.z.0)
    }

    /// Pop a Vec3 from the stack
    #[inline(always)]
    pub fn pop_vec3(&mut self) -> Result<Vec3, LpsVmError> {
        let (x, y, z) = self.pop3()?;
        Ok(Vec3::new(Dec32(x), Dec32(y), Dec32(z)))
    }

    /// Push a Vec4 onto the stack (as 4 Dec32 values)
    #[inline(always)]
    pub fn push_vec4(&mut self, v: Vec4) -> Result<(), LpsVmError> {
        self.push4(v.x.0, v.y.0, v.z.0, v.w.0)
    }

    /// Pop a Vec4 from the stack
    #[inline(always)]
    pub fn pop_vec4(&mut self) -> Result<Vec4, LpsVmError> {
        let (x, y, z, w) = self.pop4()?;
        Ok(Vec4::new(Dec32(x), Dec32(y), Dec32(z), Dec32(w)))
    }

    /// Push 9 raw i32 values onto the stack
    #[allow(clippy::too_many_arguments)]
    #[inline(always)]
    pub fn push9(
        &mut self,
        v0: i32,
        v1: i32,
        v2: i32,
        v3: i32,
        v4: i32,
        v5: i32,
        v6: i32,
        v7: i32,
        v8: i32,
    ) -> Result<(), LpsVmError> {
        if self.sp + 9 > self.max_size {
            return Err(LpsVmError::StackOverflow { sp: self.sp });
        }
        self.data[self.sp] = v0;
        self.data[self.sp + 1] = v1;
        self.data[self.sp + 2] = v2;
        self.data[self.sp + 3] = v3;
        self.data[self.sp + 4] = v4;
        self.data[self.sp + 5] = v5;
        self.data[self.sp + 6] = v6;
        self.data[self.sp + 7] = v7;
        self.data[self.sp + 8] = v8;
        self.sp += 9;
        Ok(())
    }

    /// Pop 9 raw i32 values from the stack
    #[inline(always)]
    #[allow(clippy::type_complexity)]
    pub fn pop9(&mut self) -> Result<(i32, i32, i32, i32, i32, i32, i32, i32, i32), LpsVmError> {
        if self.sp < 9 {
            return Err(LpsVmError::StackUnderflow {
                required: 9,
                actual: self.sp,
            });
        }
        self.sp -= 1;
        let i = self.data[self.sp];
        self.sp -= 1;
        let h = self.data[self.sp];
        self.sp -= 1;
        let g = self.data[self.sp];
        self.sp -= 1;
        let f = self.data[self.sp];
        self.sp -= 1;
        let e = self.data[self.sp];
        self.sp -= 1;
        let d = self.data[self.sp];
        self.sp -= 1;
        let c = self.data[self.sp];
        self.sp -= 1;
        let b = self.data[self.sp];
        self.sp -= 1;
        let a = self.data[self.sp];
        Ok((a, b, c, d, e, f, g, h, i))
    }

    /// Push a Mat3 onto the stack (as 9 Dec32 values)
    #[inline(always)]
    pub fn push_mat3(&mut self, m: Mat3) -> Result<(), LpsVmError> {
        self.push9(
            m.m[0].0, m.m[1].0, m.m[2].0, m.m[3].0, m.m[4].0, m.m[5].0, m.m[6].0, m.m[7].0,
            m.m[8].0,
        )
    }

    /// Pop a Mat3 from the stack
    #[inline(always)]
    pub fn pop_mat3(&mut self) -> Result<Mat3, LpsVmError> {
        let (m00, m10, m20, m01, m11, m21, m02, m12, m22) = self.pop9()?;
        Ok(Mat3::new(
            Dec32(m00),
            Dec32(m10),
            Dec32(m20),
            Dec32(m01),
            Dec32(m11),
            Dec32(m21),
            Dec32(m02),
            Dec32(m12),
            Dec32(m22),
        ))
    }

    // === Stack manipulation (dup/drop/swap) ===

    /// Duplicate top 1 stack value
    #[inline(always)]
    pub fn dup1(&mut self) -> Result<(), LpsVmError> {
        if self.sp < 1 {
            return Err(LpsVmError::StackUnderflow {
                required: 1,
                actual: self.sp,
            });
        }
        if self.sp >= self.max_size {
            return Err(LpsVmError::StackOverflow { sp: self.sp });
        }

        let val = self.data[self.sp - 1];
        self.data[self.sp] = val;
        self.sp += 1;

        Ok(())
    }

    /// Duplicate top 2 stack values
    #[inline(always)]
    pub fn dup2(&mut self) -> Result<(), LpsVmError> {
        if self.sp < 2 {
            return Err(LpsVmError::StackUnderflow {
                required: 2,
                actual: self.sp,
            });
        }
        if self.sp + 2 > self.max_size {
            return Err(LpsVmError::StackOverflow { sp: self.sp });
        }

        let x = self.data[self.sp - 2];
        let y = self.data[self.sp - 1];
        self.data[self.sp] = x;
        self.data[self.sp + 1] = y;
        self.sp += 2;

        Ok(())
    }

    /// Duplicate top 3 stack values
    #[inline(always)]
    pub fn dup3(&mut self) -> Result<(), LpsVmError> {
        if self.sp < 3 {
            return Err(LpsVmError::StackUnderflow {
                required: 3,
                actual: self.sp,
            });
        }
        if self.sp + 3 > self.max_size {
            return Err(LpsVmError::StackOverflow { sp: self.sp });
        }

        let x = self.data[self.sp - 3];
        let y = self.data[self.sp - 2];
        let z = self.data[self.sp - 1];
        self.data[self.sp] = x;
        self.data[self.sp + 1] = y;
        self.data[self.sp + 2] = z;
        self.sp += 3;

        Ok(())
    }

    /// Duplicate top 4 stack values
    #[inline(always)]
    pub fn dup4(&mut self) -> Result<(), LpsVmError> {
        if self.sp < 4 {
            return Err(LpsVmError::StackUnderflow {
                required: 4,
                actual: self.sp,
            });
        }
        if self.sp + 4 > self.max_size {
            return Err(LpsVmError::StackOverflow { sp: self.sp });
        }

        let x = self.data[self.sp - 4];
        let y = self.data[self.sp - 3];
        let z = self.data[self.sp - 2];
        let w = self.data[self.sp - 1];
        self.data[self.sp] = x;
        self.data[self.sp + 1] = y;
        self.data[self.sp + 2] = z;
        self.data[self.sp + 3] = w;
        self.sp += 4;

        Ok(())
    }

    /// Remove top 1 value from stack
    #[inline(always)]
    pub fn drop1(&mut self) -> Result<(), LpsVmError> {
        if self.sp < 1 {
            return Err(LpsVmError::StackUnderflow {
                required: 1,
                actual: self.sp,
            });
        }
        self.sp -= 1;
        Ok(())
    }

    /// Remove top 2 values from stack
    #[inline(always)]
    pub fn drop2(&mut self) -> Result<(), LpsVmError> {
        if self.sp < 2 {
            return Err(LpsVmError::StackUnderflow {
                required: 2,
                actual: self.sp,
            });
        }
        self.sp -= 2;
        Ok(())
    }

    /// Remove top 3 values from stack
    #[inline(always)]
    pub fn drop3(&mut self) -> Result<(), LpsVmError> {
        if self.sp < 3 {
            return Err(LpsVmError::StackUnderflow {
                required: 3,
                actual: self.sp,
            });
        }
        self.sp -= 3;
        Ok(())
    }

    /// Remove top 4 values from stack
    #[inline(always)]
    pub fn drop4(&mut self) -> Result<(), LpsVmError> {
        if self.sp < 4 {
            return Err(LpsVmError::StackUnderflow {
                required: 4,
                actual: self.sp,
            });
        }
        self.sp -= 4;
        Ok(())
    }

    /// Duplicate top 9 stack values
    #[inline(always)]
    pub fn dup9(&mut self) -> Result<(), LpsVmError> {
        if self.sp < 9 {
            return Err(LpsVmError::StackUnderflow {
                required: 9,
                actual: self.sp,
            });
        }
        if self.sp + 9 > self.max_size {
            return Err(LpsVmError::StackOverflow { sp: self.sp });
        }

        for i in 0..9 {
            self.data[self.sp + i] = self.data[self.sp - 9 + i];
        }
        self.sp += 9;

        Ok(())
    }

    /// Remove top 9 values from stack
    #[inline(always)]
    pub fn drop9(&mut self) -> Result<(), LpsVmError> {
        if self.sp < 9 {
            return Err(LpsVmError::StackUnderflow {
                required: 9,
                actual: self.sp,
            });
        }
        self.sp -= 9;
        Ok(())
    }

    /// Swap top two stack items
    #[inline(always)]
    pub fn swap(&mut self) -> Result<(), LpsVmError> {
        if self.sp < 2 {
            return Err(LpsVmError::StackUnderflow {
                required: 2,
                actual: self.sp,
            });
        }

        let a = self.data[self.sp - 2];
        let b = self.data[self.sp - 1];
        self.data[self.sp - 2] = b;
        self.data[self.sp - 1] = a;

        Ok(())
    }

    // === Swizzle operations ===

    /// Swizzle vec3 to vec2
    #[inline(always)]
    pub fn swizzle3to2(&mut self, idx0: u8, idx1: u8) -> Result<(), LpsVmError> {
        if self.sp < 3 {
            return Err(LpsVmError::StackUnderflow {
                required: 3,
                actual: self.sp,
            });
        }

        let c2 = self.data[self.sp - 1];
        let c1 = self.data[self.sp - 2];
        let c0 = self.data[self.sp - 3];

        let components = [c0, c1, c2];
        let result0 = components[idx0 as usize];
        let result1 = components[idx1 as usize];

        self.sp -= 3; // Pop all 3
        self.data[self.sp] = result0;
        self.data[self.sp + 1] = result1;
        self.sp += 2; // Push 2

        Ok(())
    }

    /// Swizzle vec3 to vec3
    #[inline(always)]
    pub fn swizzle3to3(&mut self, idx0: u8, idx1: u8, idx2: u8) -> Result<(), LpsVmError> {
        if self.sp < 3 {
            return Err(LpsVmError::StackUnderflow {
                required: 3,
                actual: self.sp,
            });
        }

        let c2 = self.data[self.sp - 1];
        let c1 = self.data[self.sp - 2];
        let c0 = self.data[self.sp - 3];

        let components = [c0, c1, c2];
        let result0 = components[idx0 as usize];
        let result1 = components[idx1 as usize];
        let result2 = components[idx2 as usize];

        self.sp -= 3;
        self.data[self.sp] = result0;
        self.data[self.sp + 1] = result1;
        self.data[self.sp + 2] = result2;
        self.sp += 3;

        Ok(())
    }

    /// Swizzle vec4 to vec2
    #[inline(always)]
    pub fn swizzle4to2(&mut self, idx0: u8, idx1: u8) -> Result<(), LpsVmError> {
        if self.sp < 4 {
            return Err(LpsVmError::StackUnderflow {
                required: 4,
                actual: self.sp,
            });
        }

        let c3 = self.data[self.sp - 1];
        let c2 = self.data[self.sp - 2];
        let c1 = self.data[self.sp - 3];
        let c0 = self.data[self.sp - 4];

        let components = [c0, c1, c2, c3];
        let result0 = components[idx0 as usize];
        let result1 = components[idx1 as usize];

        self.sp -= 4; // Pop all 4
        self.data[self.sp] = result0;
        self.data[self.sp + 1] = result1;
        self.sp += 2; // Push 2

        Ok(())
    }

    /// Swizzle vec4 to vec3
    #[inline(always)]
    pub fn swizzle4to3(&mut self, idx0: u8, idx1: u8, idx2: u8) -> Result<(), LpsVmError> {
        if self.sp < 4 {
            return Err(LpsVmError::StackUnderflow {
                required: 4,
                actual: self.sp,
            });
        }

        let c3 = self.data[self.sp - 1];
        let c2 = self.data[self.sp - 2];
        let c1 = self.data[self.sp - 3];
        let c0 = self.data[self.sp - 4];

        let components = [c0, c1, c2, c3];
        let result0 = components[idx0 as usize];
        let result1 = components[idx1 as usize];
        let result2 = components[idx2 as usize];

        self.sp -= 4;
        self.data[self.sp] = result0;
        self.data[self.sp + 1] = result1;
        self.data[self.sp + 2] = result2;
        self.sp += 3;

        Ok(())
    }

    /// Swizzle vec4 to vec4
    #[inline(always)]
    pub fn swizzle4to4(
        &mut self,
        idx0: u8,
        idx1: u8,
        idx2: u8,
        idx3: u8,
    ) -> Result<(), LpsVmError> {
        if self.sp < 4 {
            return Err(LpsVmError::StackUnderflow {
                required: 4,
                actual: self.sp,
            });
        }

        let c3 = self.data[self.sp - 1];
        let c2 = self.data[self.sp - 2];
        let c1 = self.data[self.sp - 3];
        let c0 = self.data[self.sp - 4];

        let components = [c0, c1, c2, c3];
        let result0 = components[idx0 as usize];
        let result1 = components[idx1 as usize];
        let result2 = components[idx2 as usize];
        let result3 = components[idx3 as usize];

        self.sp -= 4;
        self.data[self.sp] = result0;
        self.data[self.sp + 1] = result1;
        self.data[self.sp + 2] = result2;
        self.data[self.sp + 3] = result3;
        self.sp += 4;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use lp_math::dec32::ToDec32;

    use super::*;

    #[test]
    fn test_stack_creation() {
        let stack = ValueStack::new(64);
        assert_eq!(stack.sp(), 0);
        assert_eq!(stack.raw_slice().len(), 64);
    }

    #[test]
    fn test_push_pop_dec32() {
        let mut stack = ValueStack::new(64);

        stack.push_dec32(1.5.to_dec32()).unwrap();
        stack.push_dec32(2.5.to_dec32()).unwrap();

        assert_eq!(stack.sp(), 2);

        let b = stack.pop_dec32().unwrap();
        let a = stack.pop_dec32().unwrap();

        assert_eq!(a.to_f32(), 1.5);
        assert_eq!(b.to_f32(), 2.5);
        assert_eq!(stack.sp(), 0);
    }

    #[test]
    fn test_push_pop_int32() {
        let mut stack = ValueStack::new(64);

        stack.push_int32(42).unwrap();
        stack.push_int32(99).unwrap();

        assert_eq!(stack.sp(), 2);

        let b = stack.pop_int32().unwrap();
        let a = stack.pop_int32().unwrap();

        assert_eq!(a, 42);
        assert_eq!(b, 99);
        assert_eq!(stack.sp(), 0);
    }

    #[test]
    fn test_pop2() {
        let mut stack = ValueStack::new(64);

        stack.push_int32(10).unwrap();
        stack.push_int32(20).unwrap();

        let (a, b) = stack.pop2().unwrap();
        assert_eq!(a, 10);
        assert_eq!(b, 20);
        assert_eq!(stack.sp(), 0);
    }

    #[test]
    fn test_pop3() {
        let mut stack = ValueStack::new(64);

        stack.push_int32(10).unwrap();
        stack.push_int32(20).unwrap();
        stack.push_int32(30).unwrap();

        let (a, b, c) = stack.pop3().unwrap();
        assert_eq!(a, 10);
        assert_eq!(b, 20);
        assert_eq!(c, 30);
        assert_eq!(stack.sp(), 0);
    }

    #[test]
    fn test_pop4() {
        let mut stack = ValueStack::new(64);

        stack.push_int32(10).unwrap();
        stack.push_int32(20).unwrap();
        stack.push_int32(30).unwrap();
        stack.push_int32(40).unwrap();

        let (a, b, c, d) = stack.pop4().unwrap();
        assert_eq!(a, 10);
        assert_eq!(b, 20);
        assert_eq!(c, 30);
        assert_eq!(d, 40);
        assert_eq!(stack.sp(), 0);
    }

    #[test]
    fn test_push2() {
        let mut stack = ValueStack::new(64);

        stack.push2(10, 20).unwrap();

        assert_eq!(stack.sp(), 2);
        let (a, b) = stack.pop2().unwrap();
        assert_eq!(a, 10);
        assert_eq!(b, 20);
    }

    #[test]
    fn test_push3() {
        let mut stack = ValueStack::new(64);

        stack.push3(10, 20, 30).unwrap();

        assert_eq!(stack.sp(), 3);
        let (a, b, c) = stack.pop3().unwrap();
        assert_eq!(a, 10);
        assert_eq!(b, 20);
        assert_eq!(c, 30);
    }

    #[test]
    fn test_push4() {
        let mut stack = ValueStack::new(64);

        stack.push4(10, 20, 30, 40).unwrap();

        assert_eq!(stack.sp(), 4);
        let (a, b, c, d) = stack.pop4().unwrap();
        assert_eq!(a, 10);
        assert_eq!(b, 20);
        assert_eq!(c, 30);
        assert_eq!(d, 40);
    }

    #[test]
    fn test_push_pop_vec2() {
        let mut stack = ValueStack::new(64);

        let v = Vec2::new(1.5.to_dec32(), 2.5.to_dec32());
        stack.push_vec2(v).unwrap();

        assert_eq!(stack.sp(), 2);

        let result = stack.pop_vec2().unwrap();
        assert_eq!(result.x.to_f32(), 1.5);
        assert_eq!(result.y.to_f32(), 2.5);
        assert_eq!(stack.sp(), 0);
    }

    #[test]
    fn test_push_pop_vec3() {
        let mut stack = ValueStack::new(64);

        let v = Vec3::new(1.5.to_dec32(), 2.5.to_dec32(), 3.5.to_dec32());
        stack.push_vec3(v).unwrap();

        assert_eq!(stack.sp(), 3);

        let result = stack.pop_vec3().unwrap();
        assert_eq!(result.x.to_f32(), 1.5);
        assert_eq!(result.y.to_f32(), 2.5);
        assert_eq!(result.z.to_f32(), 3.5);
        assert_eq!(stack.sp(), 0);
    }

    #[test]
    fn test_push_pop_vec4() {
        let mut stack = ValueStack::new(64);

        let v = Vec4::new(
            1.5.to_dec32(),
            2.5.to_dec32(),
            3.5.to_dec32(),
            4.5.to_dec32(),
        );
        stack.push_vec4(v).unwrap();

        assert_eq!(stack.sp(), 4);

        let result = stack.pop_vec4().unwrap();
        assert_eq!(result.x.to_f32(), 1.5);
        assert_eq!(result.y.to_f32(), 2.5);
        assert_eq!(result.z.to_f32(), 3.5);
        assert_eq!(result.w.to_f32(), 4.5);
        assert_eq!(stack.sp(), 0);
    }

    #[test]
    fn test_stack_overflow() {
        let mut stack = ValueStack::new(2);

        stack.push_int32(1).unwrap();
        stack.push_int32(2).unwrap();

        let result = stack.push_int32(3);
        assert!(matches!(result, Err(LpsVmError::StackOverflow { sp: 2 })));
    }

    #[test]
    fn test_stack_underflow() {
        let mut stack = ValueStack::new(64);

        let result = stack.pop_int32();
        assert!(matches!(
            result,
            Err(LpsVmError::StackUnderflow {
                required: 1,
                actual: 0
            })
        ));
    }

    #[test]
    fn test_reset() {
        let mut stack = ValueStack::new(64);

        stack.push_int32(1).unwrap();
        stack.push_int32(2).unwrap();
        stack.push_int32(3).unwrap();

        assert_eq!(stack.sp(), 3);

        stack.reset();
        assert_eq!(stack.sp(), 0);
    }

    #[test]
    fn test_dup1() {
        let mut stack = ValueStack::new(64);

        stack.push_int32(42).unwrap();
        stack.dup1().unwrap();

        assert_eq!(stack.sp(), 2);
        assert_eq!(stack.raw_slice()[0], 42);
        assert_eq!(stack.raw_slice()[1], 42);
    }

    #[test]
    fn test_dup2() {
        let mut stack = ValueStack::new(64);

        stack.push_int32(10).unwrap();
        stack.push_int32(20).unwrap();
        stack.dup2().unwrap();

        assert_eq!(stack.sp(), 4);
        assert_eq!(stack.raw_slice()[0], 10);
        assert_eq!(stack.raw_slice()[1], 20);
        assert_eq!(stack.raw_slice()[2], 10);
        assert_eq!(stack.raw_slice()[3], 20);
    }

    #[test]
    fn test_dup3() {
        let mut stack = ValueStack::new(64);

        stack.push_int32(10).unwrap();
        stack.push_int32(20).unwrap();
        stack.push_int32(30).unwrap();
        stack.dup3().unwrap();

        assert_eq!(stack.sp(), 6);
        assert_eq!(stack.raw_slice()[0], 10);
        assert_eq!(stack.raw_slice()[1], 20);
        assert_eq!(stack.raw_slice()[2], 30);
        assert_eq!(stack.raw_slice()[3], 10);
        assert_eq!(stack.raw_slice()[4], 20);
        assert_eq!(stack.raw_slice()[5], 30);
    }

    #[test]
    fn test_dup4() {
        let mut stack = ValueStack::new(64);

        stack.push_int32(10).unwrap();
        stack.push_int32(20).unwrap();
        stack.push_int32(30).unwrap();
        stack.push_int32(40).unwrap();
        stack.dup4().unwrap();

        assert_eq!(stack.sp(), 8);
        assert_eq!(stack.raw_slice()[0], 10);
        assert_eq!(stack.raw_slice()[1], 20);
        assert_eq!(stack.raw_slice()[2], 30);
        assert_eq!(stack.raw_slice()[3], 40);
        assert_eq!(stack.raw_slice()[4], 10);
        assert_eq!(stack.raw_slice()[5], 20);
        assert_eq!(stack.raw_slice()[6], 30);
        assert_eq!(stack.raw_slice()[7], 40);
    }

    #[test]
    fn test_drop1() {
        let mut stack = ValueStack::new(64);

        stack.push_int32(10).unwrap();
        stack.push_int32(20).unwrap();
        stack.drop1().unwrap();

        assert_eq!(stack.sp(), 1);
        assert_eq!(stack.raw_slice()[0], 10);
    }

    #[test]
    fn test_drop2() {
        let mut stack = ValueStack::new(64);

        stack.push_int32(10).unwrap();
        stack.push_int32(20).unwrap();
        stack.push_int32(30).unwrap();
        stack.drop2().unwrap();

        assert_eq!(stack.sp(), 1);
        assert_eq!(stack.raw_slice()[0], 10);
    }

    #[test]
    fn test_drop3() {
        let mut stack = ValueStack::new(64);

        stack.push_int32(10).unwrap();
        stack.push_int32(20).unwrap();
        stack.push_int32(30).unwrap();
        stack.push_int32(40).unwrap();
        stack.drop3().unwrap();

        assert_eq!(stack.sp(), 1);
        assert_eq!(stack.raw_slice()[0], 10);
    }

    #[test]
    fn test_drop4() {
        let mut stack = ValueStack::new(64);

        stack.push_int32(10).unwrap();
        stack.push_int32(20).unwrap();
        stack.push_int32(30).unwrap();
        stack.push_int32(40).unwrap();
        stack.push_int32(50).unwrap();
        stack.drop4().unwrap();

        assert_eq!(stack.sp(), 1);
        assert_eq!(stack.raw_slice()[0], 10);
    }

    #[test]
    fn test_swap() {
        let mut stack = ValueStack::new(64);

        stack.push_int32(10).unwrap();
        stack.push_int32(20).unwrap();
        stack.swap().unwrap();

        assert_eq!(stack.sp(), 2);
        assert_eq!(stack.raw_slice()[0], 20);
        assert_eq!(stack.raw_slice()[1], 10);
    }

    #[test]
    fn test_swizzle3to2_xy() {
        let mut stack = ValueStack::new(64);

        // Push vec3(10, 20, 30)
        stack.push3(10, 20, 30).unwrap();

        // Swizzle to .xy (indices 0, 1)
        stack.swizzle3to2(0, 1).unwrap();

        assert_eq!(stack.sp(), 2);
        assert_eq!(stack.raw_slice()[0], 10);
        assert_eq!(stack.raw_slice()[1], 20);
    }

    #[test]
    fn test_swizzle3to2_yz() {
        let mut stack = ValueStack::new(64);

        // Push vec3(10, 20, 30)
        stack.push3(10, 20, 30).unwrap();

        // Swizzle to .yz (indices 1, 2)
        stack.swizzle3to2(1, 2).unwrap();

        assert_eq!(stack.sp(), 2);
        assert_eq!(stack.raw_slice()[0], 20);
        assert_eq!(stack.raw_slice()[1], 30);
    }

    #[test]
    fn test_swizzle3to3_zyx() {
        let mut stack = ValueStack::new(64);

        // Push vec3(10, 20, 30)
        stack.push3(10, 20, 30).unwrap();

        // Swizzle to .zyx (indices 2, 1, 0)
        stack.swizzle3to3(2, 1, 0).unwrap();

        assert_eq!(stack.sp(), 3);
        assert_eq!(stack.raw_slice()[0], 30);
        assert_eq!(stack.raw_slice()[1], 20);
        assert_eq!(stack.raw_slice()[2], 10);
    }

    #[test]
    fn test_swizzle4to2_xy() {
        let mut stack = ValueStack::new(64);

        // Push vec4(10, 20, 30, 40)
        stack.push4(10, 20, 30, 40).unwrap();

        // Swizzle to .xy (indices 0, 1)
        stack.swizzle4to2(0, 1).unwrap();

        assert_eq!(stack.sp(), 2);
        assert_eq!(stack.raw_slice()[0], 10);
        assert_eq!(stack.raw_slice()[1], 20);
    }

    #[test]
    fn test_swizzle4to2_zw() {
        let mut stack = ValueStack::new(64);

        // Push vec4(10, 20, 30, 40)
        stack.push4(10, 20, 30, 40).unwrap();

        // Swizzle to .zw (indices 2, 3)
        stack.swizzle4to2(2, 3).unwrap();

        assert_eq!(stack.sp(), 2);
        assert_eq!(stack.raw_slice()[0], 30);
        assert_eq!(stack.raw_slice()[1], 40);
    }

    #[test]
    fn test_swizzle4to3_xyz() {
        let mut stack = ValueStack::new(64);

        // Push vec4(10, 20, 30, 40)
        stack.push4(10, 20, 30, 40).unwrap();

        // Swizzle to .xyz (indices 0, 1, 2)
        stack.swizzle4to3(0, 1, 2).unwrap();

        assert_eq!(stack.sp(), 3);
        assert_eq!(stack.raw_slice()[0], 10);
        assert_eq!(stack.raw_slice()[1], 20);
        assert_eq!(stack.raw_slice()[2], 30);
    }

    #[test]
    fn test_swizzle4to4_wzyx() {
        let mut stack = ValueStack::new(64);

        // Push vec4(10, 20, 30, 40)
        stack.push4(10, 20, 30, 40).unwrap();

        // Swizzle to .wzyx (indices 3, 2, 1, 0)
        stack.swizzle4to4(3, 2, 1, 0).unwrap();

        assert_eq!(stack.sp(), 4);
        assert_eq!(stack.raw_slice()[0], 40);
        assert_eq!(stack.raw_slice()[1], 30);
        assert_eq!(stack.raw_slice()[2], 20);
        assert_eq!(stack.raw_slice()[3], 10);
    }

    #[test]
    fn test_dup_underflow() {
        let mut stack = ValueStack::new(64);

        let result = stack.dup1();
        assert!(matches!(
            result,
            Err(LpsVmError::StackUnderflow {
                required: 1,
                actual: 0
            })
        ));
    }

    #[test]
    fn test_drop_underflow() {
        let mut stack = ValueStack::new(64);

        let result = stack.drop1();
        assert!(matches!(
            result,
            Err(LpsVmError::StackUnderflow {
                required: 1,
                actual: 0
            })
        ));
    }

    #[test]
    fn test_swizzle_underflow() {
        let mut stack = ValueStack::new(64);

        stack.push2(10, 20).unwrap(); // Only 2 values on stack

        let result = stack.swizzle3to2(0, 1);
        assert!(matches!(
            result,
            Err(LpsVmError::StackUnderflow {
                required: 3,
                actual: 2
            })
        ));
    }
}
