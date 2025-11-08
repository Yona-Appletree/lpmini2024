/// Increment/Decrement operation code generation
extern crate alloc;

use crate::compiler::codegen::CodeGenerator;
use crate::fixed::ToFixed;
use crate::shared::Type;
use crate::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    /// Generate code for prefix increment: ++var
    /// Returns the incremented value
    pub(crate) fn gen_pre_increment(&mut self, var_name: &str, var_ty: &Type) {
        // Load variable
        self.gen_load_variable(var_name, var_ty);

        // Push 1 (Fixed or Int32)
        match var_ty {
            Type::Fixed => self.code.push(LpsOpCode::Push(1.0f32.to_fixed())),
            Type::Int32 => self.code.push(LpsOpCode::PushInt32(1)),
            _ => {} // Shouldn't happen, type checker prevents this
        }

        // Add
        match var_ty {
            Type::Fixed | Type::Int32 => self.code.push(LpsOpCode::AddFixed),
            _ => {}
        }

        // Duplicate for return value
        self.code.push(LpsOpCode::Dup1);

        // Store back to variable
        self.gen_store_variable(var_name, var_ty);
    }

    /// Generate code for prefix decrement: --var
    /// Returns the decremented value
    pub(crate) fn gen_pre_decrement(&mut self, var_name: &str, var_ty: &Type) {
        // Load variable
        self.gen_load_variable(var_name, var_ty);

        // Push 1 (Fixed or Int32)
        match var_ty {
            Type::Fixed => self.code.push(LpsOpCode::Push(1.0f32.to_fixed())),
            Type::Int32 => self.code.push(LpsOpCode::PushInt32(1)),
            _ => {}
        }

        // Subtract
        match var_ty {
            Type::Fixed | Type::Int32 => self.code.push(LpsOpCode::SubFixed),
            _ => {}
        }

        // Duplicate for return value
        self.code.push(LpsOpCode::Dup1);

        // Store back to variable
        self.gen_store_variable(var_name, var_ty);
    }

    /// Generate code for postfix increment: var++
    /// Returns the original value (before increment)
    pub(crate) fn gen_post_increment(&mut self, var_name: &str, var_ty: &Type) {
        // Load variable (original value)
        self.gen_load_variable(var_name, var_ty);

        // Duplicate for return value
        self.code.push(LpsOpCode::Dup1);

        // Push 1 (Fixed or Int32)
        match var_ty {
            Type::Fixed => self.code.push(LpsOpCode::Push(1.0f32.to_fixed())),
            Type::Int32 => self.code.push(LpsOpCode::PushInt32(1)),
            _ => {}
        }

        // Add
        match var_ty {
            Type::Fixed | Type::Int32 => self.code.push(LpsOpCode::AddFixed),
            _ => {}
        }

        // Store back to variable
        self.gen_store_variable(var_name, var_ty);
    }

    /// Generate code for postfix decrement: var--
    /// Returns the original value (before decrement)
    pub(crate) fn gen_post_decrement(&mut self, var_name: &str, var_ty: &Type) {
        // Load variable (original value)
        self.gen_load_variable(var_name, var_ty);

        // Duplicate for return value
        self.code.push(LpsOpCode::Dup1);

        // Push 1 (Fixed or Int32)
        match var_ty {
            Type::Fixed => self.code.push(LpsOpCode::Push(1.0f32.to_fixed())),
            Type::Int32 => self.code.push(LpsOpCode::PushInt32(1)),
            _ => {}
        }

        // Subtract
        match var_ty {
            Type::Fixed | Type::Int32 => self.code.push(LpsOpCode::SubFixed),
            _ => {}
        }

        // Store back to variable
        self.gen_store_variable(var_name, var_ty);
    }

    /// Helper to load a variable onto the stack
    fn gen_load_variable(&mut self, var_name: &str, var_ty: &Type) {
        if let Some(local_idx) = self.locals.get(var_name) {
            match var_ty {
                Type::Fixed => self.code.push(LpsOpCode::LoadLocalFixed(local_idx)),
                Type::Int32 => self.code.push(LpsOpCode::LoadLocalInt32(local_idx)),
                Type::Vec2 => self.code.push(LpsOpCode::LoadLocalVec2(local_idx)),
                Type::Vec3 => self.code.push(LpsOpCode::LoadLocalVec3(local_idx)),
                Type::Vec4 => self.code.push(LpsOpCode::LoadLocalVec4(local_idx)),
                _ => {}
            }
        }
    }

    /// Helper to store a value from the stack into a variable
    fn gen_store_variable(&mut self, var_name: &str, var_ty: &Type) {
        if let Some(local_idx) = self.locals.get(var_name) {
            match var_ty {
                Type::Fixed => self.code.push(LpsOpCode::StoreLocalFixed(local_idx)),
                Type::Int32 => self.code.push(LpsOpCode::StoreLocalInt32(local_idx)),
                Type::Vec2 => self.code.push(LpsOpCode::StoreLocalVec2(local_idx)),
                Type::Vec3 => self.code.push(LpsOpCode::StoreLocalVec3(local_idx)),
                Type::Vec4 => self.code.push(LpsOpCode::StoreLocalVec4(local_idx)),
                _ => {}
            }
        }
    }
}
