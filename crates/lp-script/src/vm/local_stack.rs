/// Local variable storage for LPS VM (optimized with raw i32 array)
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

use super::error::LpsVmError;
use super::lps_program::LocalVarDef;
use crate::fixed::Fixed;
use crate::shared::Type;

impl LocalStack {
    /// Create new locals storage with the given capacity (in i32 units)
    ///
    /// Capacity should be large enough for all locals across max call depth.
    /// Example: if max depth is 64 and average function uses 20 i32s of locals,
    /// capacity should be at least 64 * 20 = 1280.
    pub fn new(capacity: usize) -> Self {
        let mut data = Vec::new();
        data.resize(capacity, 0);

        LocalStack {
            data,
            metadata: Vec::new(),
            capacity,
            sp: 0,
            local_count: 0,
        }
    }

    /// Allocate space for a batch of locals (typically for a function)
    ///
    /// Returns the base local index for this batch.
    /// The caller should track this to calculate absolute indices.
    pub fn allocate_locals(&mut self, defs: &[LocalVarDef]) -> Result<usize, LpsVmError> {
        let base_local_idx = self.local_count;

        for def in defs {
            let size = def.ty.size_in_i32s();
            let offset = self.sp;

            // Check capacity
            if self.sp + size > self.capacity {
                return Err(LpsVmError::LocalOutOfBounds {
                    local_idx: self.local_count,
                    max: self.capacity,
                });
            }

            // Add metadata
            self.metadata.push(LocalMetadata {
                name: def.name.clone(),
                ty: def.ty.clone(),
                offset,
                size,
            });

            // Initialize with provided value or zero
            if let Some(ref init_value) = def.initial_value {
                // Use provided initial value
                for (i, &val) in init_value.iter().enumerate() {
                    if i < size {
                        self.data[offset + i] = val;
                    }
                }
                // Fill remaining with zeros if init_value is shorter
                for i in init_value.len()..size {
                    self.data[offset + i] = 0;
                }
            } else {
                // Initialize to zero
                for i in 0..size {
                    self.data[offset + i] = 0;
                }
            }

            self.sp += size;
            self.local_count += 1;
        }

        Ok(base_local_idx)
    }

    /// Deallocate locals back to a previous stack pointer
    ///
    /// Used when returning from a function to restore the previous frame.
    pub fn deallocate_to(&mut self, local_idx: usize) {
        if local_idx < self.local_count {
            // Get the offset to restore to
            if local_idx == 0 {
                self.sp = 0;
            } else if let Some(meta) = self.metadata.get(local_idx) {
                self.sp = meta.offset;
            }

            // Remove metadata for deallocated locals
            self.metadata.truncate(local_idx);
            self.local_count = local_idx;
        }
    }

    /// Reset locals to a given count and re-initialize their values
    ///
    /// Used when resetting the VM for a new execution run.
    pub fn reset_locals(
        &mut self,
        target_local_count: usize,
        defs: &[LocalVarDef],
    ) -> Result<(), LpsVmError> {
        // First deallocate any extra locals
        self.deallocate_to(target_local_count);

        // Then re-initialize the values for all remaining locals
        let count = target_local_count.min(defs.len()).min(self.local_count);
        for idx in 0..count {
            let def = &defs[idx];
            let meta = self.get_metadata(idx)?;
            let offset = meta.offset;
            let size = meta.size;

            if let Some(ref init_value) = def.initial_value {
                // Use provided initial value
                for (i, &val) in init_value.iter().enumerate() {
                    if i < size {
                        self.data[offset + i] = val;
                    }
                }
                // Fill remaining with zeros if init_value is shorter
                for i in init_value.len()..size {
                    self.data[offset + i] = 0;
                }
            } else {
                // Initialize to zero
                for i in 0..size {
                    self.data[offset + i] = 0;
                }
            }
        }

        Ok(())
    }

    /// Get a Fixed value from a local (absolute index)
    #[inline(always)]
    pub fn get_fixed(&self, idx: usize) -> Result<Fixed, LpsVmError> {
        let meta = self.get_metadata(idx)?;

        if meta.ty != Type::Fixed && meta.ty != Type::Bool {
            return Err(LpsVmError::TypeMismatch);
        }

        Ok(Fixed(self.data[meta.offset]))
    }

    /// Set a Fixed value to a local (absolute index)
    #[inline(always)]
    pub fn set_fixed(&mut self, idx: usize, value: Fixed) -> Result<(), LpsVmError> {
        let (offset, ty) = {
            let meta = self.get_metadata(idx)?;
            (meta.offset, meta.ty.clone())
        };

        if ty != Type::Fixed && ty != Type::Bool {
            return Err(LpsVmError::TypeMismatch);
        }

        self.data[offset] = value.0;
        Ok(())
    }

    /// Get an Int32 value from a local (absolute index)
    #[inline(always)]
    pub fn get_int32(&self, idx: usize) -> Result<i32, LpsVmError> {
        let meta = self.get_metadata(idx)?;

        if meta.ty != Type::Int32 {
            return Err(LpsVmError::TypeMismatch);
        }

        Ok(self.data[meta.offset])
    }

    /// Set an Int32 value to a local (absolute index)
    #[inline(always)]
    pub fn set_int32(&mut self, idx: usize, value: i32) -> Result<(), LpsVmError> {
        let (offset, ty) = {
            let meta = self.get_metadata(idx)?;
            (meta.offset, meta.ty.clone())
        };

        if ty != Type::Int32 {
            return Err(LpsVmError::TypeMismatch);
        }

        self.data[offset] = value;
        Ok(())
    }

    /// Get a Vec2 value from a local (absolute index)
    #[inline(always)]
    pub fn get_vec2(&self, idx: usize) -> Result<(Fixed, Fixed), LpsVmError> {
        let meta = self.get_metadata(idx)?;

        if meta.ty != Type::Vec2 {
            return Err(LpsVmError::TypeMismatch);
        }

        let x = Fixed(self.data[meta.offset]);
        let y = Fixed(self.data[meta.offset + 1]);
        Ok((x, y))
    }

    /// Set a Vec2 value to a local (absolute index)
    #[inline(always)]
    pub fn set_vec2(&mut self, idx: usize, x: Fixed, y: Fixed) -> Result<(), LpsVmError> {
        let (offset, ty) = {
            let meta = self.get_metadata(idx)?;
            (meta.offset, meta.ty.clone())
        };

        if ty != Type::Vec2 {
            return Err(LpsVmError::TypeMismatch);
        }

        self.data[offset] = x.0;
        self.data[offset + 1] = y.0;
        Ok(())
    }

    /// Get a Vec3 value from a local (absolute index)
    #[inline(always)]
    pub fn get_vec3(&self, idx: usize) -> Result<(Fixed, Fixed, Fixed), LpsVmError> {
        let meta = self.get_metadata(idx)?;

        if meta.ty != Type::Vec3 {
            return Err(LpsVmError::TypeMismatch);
        }

        let x = Fixed(self.data[meta.offset]);
        let y = Fixed(self.data[meta.offset + 1]);
        let z = Fixed(self.data[meta.offset + 2]);
        Ok((x, y, z))
    }

    /// Set a Vec3 value to a local (absolute index)
    #[inline(always)]
    pub fn set_vec3(&mut self, idx: usize, x: Fixed, y: Fixed, z: Fixed) -> Result<(), LpsVmError> {
        let (offset, ty) = {
            let meta = self.get_metadata(idx)?;
            (meta.offset, meta.ty.clone())
        };

        if ty != Type::Vec3 {
            return Err(LpsVmError::TypeMismatch);
        }

        self.data[offset] = x.0;
        self.data[offset + 1] = y.0;
        self.data[offset + 2] = z.0;
        Ok(())
    }

    /// Get a Vec4 value from a local (absolute index)
    #[inline(always)]
    pub fn get_vec4(&self, idx: usize) -> Result<(Fixed, Fixed, Fixed, Fixed), LpsVmError> {
        let meta = self.get_metadata(idx)?;

        if meta.ty != Type::Vec4 {
            return Err(LpsVmError::TypeMismatch);
        }

        let x = Fixed(self.data[meta.offset]);
        let y = Fixed(self.data[meta.offset + 1]);
        let z = Fixed(self.data[meta.offset + 2]);
        let w = Fixed(self.data[meta.offset + 3]);
        Ok((x, y, z, w))
    }

    /// Set a Vec4 value to a local (absolute index)
    #[inline(always)]
    pub fn set_vec4(
        &mut self,
        idx: usize,
        x: Fixed,
        y: Fixed,
        z: Fixed,
        w: Fixed,
    ) -> Result<(), LpsVmError> {
        let (offset, ty) = {
            let meta = self.get_metadata(idx)?;
            (meta.offset, meta.ty.clone())
        };

        if ty != Type::Vec4 {
            return Err(LpsVmError::TypeMismatch);
        }

        self.data[offset] = x.0;
        self.data[offset + 1] = y.0;
        self.data[offset + 2] = z.0;
        self.data[offset + 3] = w.0;
        Ok(())
    }

    /// Get metadata for a local (private helper)
    #[inline(always)]
    fn get_metadata(&self, idx: usize) -> Result<&LocalMetadata, LpsVmError> {
        self.metadata.get(idx).ok_or(LpsVmError::LocalOutOfBounds {
            local_idx: idx,
            max: self.local_count,
        })
    }

    /// Get current local count
    pub fn local_count(&self) -> usize {
        self.local_count
    }

    /// Get current stack pointer (in i32 units)
    pub fn sp(&self) -> usize {
        self.sp
    }

    /// Get capacity (in i32 units)
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Get local name for debugging
    pub fn get_local_name(&self, idx: usize) -> Option<&str> {
        self.metadata.get(idx).map(|m| m.name.as_str())
    }

    /// Get local type for debugging
    pub fn get_local_type(&self, idx: usize) -> Option<&Type> {
        self.metadata.get(idx).map(|m| &m.ty)
    }

    // ============================================================================
    // Debugging API - Get values by name or index
    // ============================================================================

    /// Get a local value by name (for debugging/testing)
    /// Returns the value as a vector of i32s (raw representation)
    pub fn get_by_name(&self, name: &str) -> Option<Vec<i32>> {
        let (offset, size) = self
            .metadata
            .iter()
            .find(|m| m.name == name)
            .map(|m| (m.offset, m.size))?;

        Some(self.data[offset..offset + size].to_vec())
    }

    /// Get a Fixed local by name (for debugging/testing)
    pub fn get_fixed_by_name(&self, name: &str) -> Option<Fixed> {
        let meta = self.metadata.iter().find(|m| m.name == name)?;
        if meta.ty == Type::Fixed || meta.ty == Type::Bool {
            Some(Fixed(self.data[meta.offset]))
        } else {
            None
        }
    }

    /// Get an Int32 local by name (for debugging/testing)
    pub fn get_int32_by_name(&self, name: &str) -> Option<i32> {
        let meta = self.metadata.iter().find(|m| m.name == name)?;
        if meta.ty == Type::Int32 {
            Some(self.data[meta.offset])
        } else {
            None
        }
    }

    /// Get a Vec2 local by name (for debugging/testing)
    pub fn get_vec2_by_name(&self, name: &str) -> Option<(Fixed, Fixed)> {
        let meta = self.metadata.iter().find(|m| m.name == name)?;
        if meta.ty == Type::Vec2 {
            let x = Fixed(self.data[meta.offset]);
            let y = Fixed(self.data[meta.offset + 1]);
            Some((x, y))
        } else {
            None
        }
    }

    /// Get a Vec3 local by name (for debugging/testing)
    pub fn get_vec3_by_name(&self, name: &str) -> Option<(Fixed, Fixed, Fixed)> {
        let meta = self.metadata.iter().find(|m| m.name == name)?;
        if meta.ty == Type::Vec3 {
            let x = Fixed(self.data[meta.offset]);
            let y = Fixed(self.data[meta.offset + 1]);
            let z = Fixed(self.data[meta.offset + 2]);
            Some((x, y, z))
        } else {
            None
        }
    }

    /// Get a Vec4 local by name (for debugging/testing)
    pub fn get_vec4_by_name(&self, name: &str) -> Option<(Fixed, Fixed, Fixed, Fixed)> {
        let meta = self.metadata.iter().find(|m| m.name == name)?;
        if meta.ty == Type::Vec4 {
            let x = Fixed(self.data[meta.offset]);
            let y = Fixed(self.data[meta.offset + 1]);
            let z = Fixed(self.data[meta.offset + 2]);
            let w = Fixed(self.data[meta.offset + 3]);
            Some((x, y, z, w))
        } else {
            None
        }
    }

    /// List all locals with their names and types (for debugging)
    pub fn list_locals(&self) -> Vec<(String, Type)> {
        self.metadata
            .iter()
            .map(|m| (m.name.clone(), m.ty.clone()))
            .collect()
    }
}

/// Metadata for a single local variable
///
/// Maps a logical local index to its physical location and type in the i32 array.
/// This enables efficient storage: Fixed uses 1 i32, Vec2 uses 2, Vec4 uses 4, etc.
#[derive(Debug, Clone)]
struct LocalMetadata {
    name: String,  // For debugging
    ty: Type,      // Type of this local
    offset: usize, // Offset in i32 array where data starts
    size: usize,   // Size in i32 units
}

/// Storage for local variables with optimized memory layout
///
/// Uses a raw i32 array instead of enum variants to minimize wasted space.
/// Each local variable is allocated based on its actual size:
/// - Fixed: 1 i32
/// - Vec2: 2 i32s
/// - Vec3: 3 i32s
/// - Vec4: 4 i32s
///
/// Locals are allocated in a stack-like manner as functions are called,
/// and deallocated when functions return.
pub struct LocalStack {
    data: Vec<i32>,               // Raw i32 storage
    metadata: Vec<LocalMetadata>, // Per-local type info (indexed by absolute local idx)
    capacity: usize,              // Max i32s available
    sp: usize,                    // Current stack pointer (in i32s)
    local_count: usize,           // Number of logical locals allocated
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixed::ToFixed;
    use crate::vm::lps_program::LocalVarDef;

    #[test]
    fn test_locals_storage_creation() {
        let storage = LocalStack::new(2048);
        assert_eq!(storage.capacity(), 2048);
        assert_eq!(storage.local_count(), 0);
        assert_eq!(storage.sp(), 0);
    }

    #[test]
    fn test_allocate_deallocate() {
        let mut storage = LocalStack::new(1024);

        // Allocate 3 locals: Fixed, Vec2, Vec4
        let defs = vec![
            LocalVarDef::new("x".into(), Type::Fixed),
            LocalVarDef::new("v".into(), Type::Vec2),
            LocalVarDef::new("color".into(), Type::Vec4),
        ];

        let base = storage.allocate_locals(&defs).unwrap();
        assert_eq!(base, 0);
        assert_eq!(storage.local_count(), 3);
        assert_eq!(storage.sp(), 7); // 1 + 2 + 4 = 7 i32s

        // Deallocate back to base
        storage.deallocate_to(0);
        assert_eq!(storage.local_count(), 0);
        assert_eq!(storage.sp(), 0);
    }

    #[test]
    fn test_fixed_get_set() {
        let mut storage = LocalStack::new(64);

        let defs = vec![LocalVarDef::new("x".into(), Type::Fixed)];
        storage.allocate_locals(&defs).unwrap();

        storage.set_fixed(0, 3.14.to_fixed()).unwrap();
        let val = storage.get_fixed(0).unwrap();
        assert!((val.to_f32() - 3.14).abs() < 0.01);
    }

    #[test]
    fn test_int32_get_set() {
        let mut storage = LocalStack::new(64);

        let defs = vec![LocalVarDef::new("count".into(), Type::Int32)];
        storage.allocate_locals(&defs).unwrap();

        storage.set_int32(0, 42).unwrap();
        let val = storage.get_int32(0).unwrap();
        assert_eq!(val, 42);
    }

    #[test]
    fn test_vec2_get_set() {
        let mut storage = LocalStack::new(64);

        let defs = vec![LocalVarDef::new("pos".into(), Type::Vec2)];
        storage.allocate_locals(&defs).unwrap();

        storage.set_vec2(0, 1.0.to_fixed(), 2.0.to_fixed()).unwrap();
        let (x, y) = storage.get_vec2(0).unwrap();
        assert_eq!(x.to_f32(), 1.0);
        assert_eq!(y.to_f32(), 2.0);
    }

    #[test]
    fn test_vec3_get_set() {
        let mut storage = LocalStack::new(64);

        let defs = vec![LocalVarDef::new("pos".into(), Type::Vec3)];
        storage.allocate_locals(&defs).unwrap();

        storage
            .set_vec3(0, 1.0.to_fixed(), 2.0.to_fixed(), 3.0.to_fixed())
            .unwrap();
        let (x, y, z) = storage.get_vec3(0).unwrap();
        assert_eq!(x.to_f32(), 1.0);
        assert_eq!(y.to_f32(), 2.0);
        assert_eq!(z.to_f32(), 3.0);
    }

    #[test]
    fn test_vec4_get_set() {
        let mut storage = LocalStack::new(64);

        let defs = vec![LocalVarDef::new("color".into(), Type::Vec4)];
        storage.allocate_locals(&defs).unwrap();

        storage
            .set_vec4(
                0,
                1.0.to_fixed(),
                2.0.to_fixed(),
                3.0.to_fixed(),
                4.0.to_fixed(),
            )
            .unwrap();
        let (x, y, z, w) = storage.get_vec4(0).unwrap();
        assert_eq!(x.to_f32(), 1.0);
        assert_eq!(y.to_f32(), 2.0);
        assert_eq!(z.to_f32(), 3.0);
        assert_eq!(w.to_f32(), 4.0);
    }

    #[test]
    fn test_multiple_locals_layout() {
        let mut storage = LocalStack::new(1024);

        // Allocate: Fixed at offset 0, Vec2 at offset 1-2, Fixed at offset 3
        let defs = vec![
            LocalVarDef::new("x".into(), Type::Fixed),
            LocalVarDef::new("pos".into(), Type::Vec2),
            LocalVarDef::new("y".into(), Type::Fixed),
        ];

        storage.allocate_locals(&defs).unwrap();

        // Set values
        storage.set_fixed(0, 10.0.to_fixed()).unwrap();
        storage
            .set_vec2(1, 20.0.to_fixed(), 30.0.to_fixed())
            .unwrap();
        storage.set_fixed(2, 40.0.to_fixed()).unwrap();

        // Verify values
        assert_eq!(storage.get_fixed(0).unwrap().to_f32(), 10.0);
        let (x, y) = storage.get_vec2(1).unwrap();
        assert_eq!(x.to_f32(), 20.0);
        assert_eq!(y.to_f32(), 30.0);
        assert_eq!(storage.get_fixed(2).unwrap().to_f32(), 40.0);
    }

    #[test]
    fn test_type_mismatch() {
        let mut storage = LocalStack::new(64);

        let defs = vec![LocalVarDef::new("x".into(), Type::Fixed)];
        storage.allocate_locals(&defs).unwrap();

        // Try to access as Int32 when it's Fixed
        let result = storage.get_int32(0);
        assert!(matches!(result, Err(LpsVmError::TypeMismatch)));
    }

    #[test]
    fn test_out_of_bounds() {
        let mut storage = LocalStack::new(64);

        let defs = vec![LocalVarDef::new("x".into(), Type::Fixed)];
        storage.allocate_locals(&defs).unwrap();

        // Try to access non-existent local
        let result = storage.get_fixed(10);
        assert!(matches!(
            result,
            Err(LpsVmError::LocalOutOfBounds {
                local_idx: 10,
                max: 1
            })
        ));
    }

    #[test]
    fn test_frame_simulation() {
        let mut storage = LocalStack::new(1024);

        // Main function: Fixed, Vec2
        let main_defs = vec![
            LocalVarDef::new("time".into(), Type::Fixed),
            LocalVarDef::new("resolution".into(), Type::Vec2),
        ];
        let main_base = storage.allocate_locals(&main_defs).unwrap();
        assert_eq!(main_base, 0);
        assert_eq!(storage.sp(), 3); // 1 + 2 = 3

        // Function call: allocate Vec4, Int32
        let func_defs = vec![
            LocalVarDef::new("color".into(), Type::Vec4),
            LocalVarDef::new("index".into(), Type::Int32),
        ];
        let func_base = storage.allocate_locals(&func_defs).unwrap();
        assert_eq!(func_base, 2); // After main's 2 locals
        assert_eq!(storage.sp(), 8); // 3 + 4 + 1 = 8

        // Set values in function frame
        storage
            .set_vec4(
                2,
                1.0.to_fixed(),
                0.0.to_fixed(),
                0.0.to_fixed(),
                1.0.to_fixed(),
            )
            .unwrap();
        storage.set_int32(3, 99).unwrap();

        // Verify function values
        let (r, _g, _b, _a) = storage.get_vec4(2).unwrap();
        assert_eq!(r.to_f32(), 1.0);
        assert_eq!(storage.get_int32(3).unwrap(), 99);

        // Return from function
        storage.deallocate_to(func_base);
        assert_eq!(storage.local_count(), 2);
        assert_eq!(storage.sp(), 3);

        // Main locals should still be accessible
        storage.set_fixed(0, 1.5.to_fixed()).unwrap();
        assert_eq!(storage.get_fixed(0).unwrap().to_f32(), 1.5);
    }

    #[test]
    fn test_debugging_api_by_name() {
        let mut storage = LocalStack::new(1024);

        // Allocate locals with names
        let defs = vec![
            LocalVarDef::new("time".into(), Type::Fixed),
            LocalVarDef::new("position".into(), Type::Vec2),
            LocalVarDef::new("count".into(), Type::Int32),
            LocalVarDef::new("color".into(), Type::Vec4),
        ];
        storage.allocate_locals(&defs).unwrap();

        // Set values
        storage.set_fixed(0, 1.5.to_fixed()).unwrap();
        storage
            .set_vec2(1, 10.0.to_fixed(), 20.0.to_fixed())
            .unwrap();
        storage.set_int32(2, 42).unwrap();
        storage
            .set_vec4(
                3,
                1.0.to_fixed(),
                0.5.to_fixed(),
                0.25.to_fixed(),
                1.0.to_fixed(),
            )
            .unwrap();

        // Get by name
        assert_eq!(storage.get_fixed_by_name("time").unwrap().to_f32(), 1.5);

        let (x, y) = storage.get_vec2_by_name("position").unwrap();
        assert_eq!(x.to_f32(), 10.0);
        assert_eq!(y.to_f32(), 20.0);

        assert_eq!(storage.get_int32_by_name("count").unwrap(), 42);

        let (r, g, b, a) = storage.get_vec4_by_name("color").unwrap();
        assert_eq!(r.to_f32(), 1.0);
        assert_eq!(g.to_f32(), 0.5);
        assert_eq!(b.to_f32(), 0.25);
        assert_eq!(a.to_f32(), 1.0);

        // Test not found
        assert!(storage.get_fixed_by_name("nonexistent").is_none());
    }

    #[test]
    fn test_list_locals() {
        let mut storage = LocalStack::new(1024);

        let defs = vec![
            LocalVarDef::new("x".into(), Type::Fixed),
            LocalVarDef::new("pos".into(), Type::Vec2),
            LocalVarDef::new("count".into(), Type::Int32),
        ];
        storage.allocate_locals(&defs).unwrap();

        let locals_list = storage.list_locals();
        assert_eq!(locals_list.len(), 3);
        assert_eq!(locals_list[0].0, "x");
        assert_eq!(locals_list[0].1, Type::Fixed);
        assert_eq!(locals_list[1].0, "pos");
        assert_eq!(locals_list[1].1, Type::Vec2);
        assert_eq!(locals_list[2].0, "count");
        assert_eq!(locals_list[2].1, Type::Int32);
    }
}
