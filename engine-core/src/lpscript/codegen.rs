/// Code generator: converts AST to VM opcodes
extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::string::String;

use super::ast::{Expr, ExprKind, Stmt, StmtKind, Program, FunctionDef};
use super::error::Type;
use super::vm::opcodes::LpsOpCode;
use crate::test_engine::LoadSource;
use crate::math::ToFixed;

pub struct CodeGenerator;

/// Binary operation types
enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

/// Local variable allocation state
struct LocalAllocator {
    locals: BTreeMap<String, u32>,
    next_index: u32,
}

impl LocalAllocator {
    fn new() -> Self {
        LocalAllocator {
            locals: BTreeMap::new(),
            next_index: 0,
        }
    }
    
    fn allocate(&mut self, name: String) -> u32 {
        if let Some(&index) = self.locals.get(&name) {
            index
        } else {
            let index = self.next_index;
            self.next_index += 1;
            self.locals.insert(name, index);
            index
        }
    }
    
    fn get(&self, name: &str) -> Option<u32> {
        self.locals.get(name).copied()
    }
}

impl CodeGenerator {
    /// Generate opcodes for an expression (expression mode)
    pub fn generate(expr: &Expr) -> Vec<LpsOpCode> {
        let mut code = Vec::new();
        let mut locals = LocalAllocator::new();
        let func_offsets = BTreeMap::new(); // Empty for expression mode
        Self::gen_expr_with_locals(expr, &mut code, &mut locals, &func_offsets);
        code.push(LpsOpCode::Return);
        code
    }
    
    // Legacy gen_expr for backward compatibility
    fn gen_expr(expr: &Expr, code: &mut Vec<LpsOpCode>) {
        let mut locals = LocalAllocator::new();
        let func_offsets = BTreeMap::new();
        Self::gen_expr_with_locals(expr, code, &mut locals, &func_offsets);
    }
    
    /// Generate opcodes for a program (script mode)
    /// Returns (opcodes, local_count) tuple
    pub fn generate_program(program: &Program) -> (Vec<LpsOpCode>, u32) {
        let mut code = Vec::new();
        let mut function_offsets = BTreeMap::new();
        
        // If there are functions, emit a jump to skip them (go to main code)
        let main_jump_index = if !program.functions.is_empty() {
            code.push(LpsOpCode::Jump(0)); // Placeholder, will patch later
            Some(0)
        } else {
            None
        };
        
        // Generate code for each function
        for func in &program.functions {
            let func_start = code.len();
            function_offsets.insert(func.name.clone(), func_start as u32);
            
            let mut locals = LocalAllocator::new();
            
            // Allocate space for parameters (they're passed on stack)
            for (i, param) in func.params.iter().enumerate() {
                locals.allocate(param.name.clone());
                // Parameters are already on stack, need to store them
                match param.ty {
                    Type::Fixed | Type::Int32 => code.push(LpsOpCode::StoreLocalFixed(i as u32)),
                    Type::Vec2 => code.push(LpsOpCode::StoreLocalVec2(i as u32)),
                    Type::Vec3 => code.push(LpsOpCode::StoreLocalVec3(i as u32)),
                    Type::Vec4 => code.push(LpsOpCode::StoreLocalVec4(i as u32)),
                    Type::Void => {}
                }
            }
            
            // Generate function body
            for stmt in &func.body {
                Self::gen_stmt(stmt, &mut code, &mut locals, &function_offsets);
            }
            
            // If no explicit return, add a default one
            if !matches!(code.last(), Some(LpsOpCode::Return)) {
                if func.return_type == Type::Void {
                    code.push(LpsOpCode::Return);
                } else {
                    code.push(LpsOpCode::Push(crate::math::Fixed::ZERO));
                    code.push(LpsOpCode::Return);
                }
            }
        }
        
        // Patch the main jump to point here
        if let Some(jump_idx) = main_jump_index {
            let main_start = code.len();
            if let LpsOpCode::Jump(ref mut offset) = code[jump_idx] {
                *offset = (main_start as i32) - 1;
            }
        }
        
        // Generate main code
        let mut locals = LocalAllocator::new();
        for stmt in &program.stmts {
            Self::gen_stmt(stmt, &mut code, &mut locals, &function_offsets);
        }
        
        // If no explicit return, add one
        if !matches!(code.last(), Some(LpsOpCode::Return)) {
            code.push(LpsOpCode::Push(crate::math::Fixed::ZERO));
            code.push(LpsOpCode::Return);
        }
        
        // Return opcodes and the total number of locals allocated
        (code, locals.next_index)
    }
    
    /// Generate code for a statement
    fn gen_stmt(stmt: &Stmt, code: &mut Vec<LpsOpCode>, locals: &mut LocalAllocator, func_offsets: &BTreeMap<String, u32>) {
        match &stmt.kind {
            StmtKind::VarDecl { ty, name, init } => {
                let index = locals.allocate(name.clone());
                
                if let Some(init_expr) = init {
                    // Generate code to evaluate initializer
                    Self::gen_expr_with_locals(init_expr, code, locals, func_offsets);
                    
                    // Store in local variable
                    match ty {
                        Type::Fixed | Type::Int32 => code.push(LpsOpCode::StoreLocalFixed(index)),
                        Type::Vec2 => code.push(LpsOpCode::StoreLocalVec2(index)),
                        Type::Vec3 => code.push(LpsOpCode::StoreLocalVec3(index)),
                        Type::Vec4 => code.push(LpsOpCode::StoreLocalVec4(index)),
                        _ => {}
                    }
                }
            }
            
            StmtKind::Assignment { name, value } => {
                // Generate code for value
                Self::gen_expr_with_locals(value, code, locals, func_offsets);
                
                // Store in variable
                if let Some(index) = locals.get(name) {
                    let ty = value.ty.as_ref().unwrap();
                    match ty {
                        Type::Fixed | Type::Int32 => code.push(LpsOpCode::StoreLocalFixed(index)),
                        Type::Vec2 => code.push(LpsOpCode::StoreLocalVec2(index)),
                        Type::Vec3 => code.push(LpsOpCode::StoreLocalVec3(index)),
                        Type::Vec4 => code.push(LpsOpCode::StoreLocalVec4(index)),
                        _ => {}
                    }
                }
            }
            
            StmtKind::Return(expr) => {
                Self::gen_expr_with_locals(expr, code, locals, func_offsets);
                code.push(LpsOpCode::Return);
            }
            
            StmtKind::Expr(expr) => {
                Self::gen_expr_with_locals(expr, code, locals, func_offsets);
                // Pop the result (expression statement doesn't use the value)
                code.push(LpsOpCode::Drop);
            }
            
            StmtKind::Block(stmts) => {
                for stmt in stmts {
                    Self::gen_stmt(stmt, code, locals, func_offsets);
                }
            }
            
            StmtKind::If { condition, then_stmt, else_stmt } => {
                // Generate: condition → JumpIfZero(else_offset) → then_block → Jump(end_offset) → else_block
                Self::gen_expr_with_locals(condition, code, locals, func_offsets);
                
                // Placeholder for JumpIfZero - we'll patch the offset later
                let jump_to_else_index = code.len();
                code.push(LpsOpCode::JumpIfZero(0)); // Placeholder offset
                
                // Generate then block
                Self::gen_stmt(then_stmt, code, locals, func_offsets);
                
                if let Some(else_s) = else_stmt {
                    // Placeholder for Jump past else block
                    let jump_to_end_index = code.len();
                    code.push(LpsOpCode::Jump(0)); // Placeholder offset
                    
                    // Patch the JumpIfZero to point here (start of else block)
                    let else_start = code.len();
                    if let LpsOpCode::JumpIfZero(ref mut offset) = code[jump_to_else_index] {
                        *offset = (else_start as i32) - (jump_to_else_index as i32) - 1;
                    }
                    
                    // Generate else block
                    Self::gen_stmt(else_s, code, locals, func_offsets);
                    
                    // Patch the Jump to point here (end)
                    let end = code.len();
                    if let LpsOpCode::Jump(ref mut offset) = code[jump_to_end_index] {
                        *offset = (end as i32) - (jump_to_end_index as i32) - 1;
                    }
                } else {
                    // No else block - patch JumpIfZero to point to end
                    let end = code.len();
                    if let LpsOpCode::JumpIfZero(ref mut offset) = code[jump_to_else_index] {
                        *offset = (end as i32) - (jump_to_else_index as i32) - 1;
                    }
                }
            }
            
            StmtKind::While { condition, body } => {
                // Generate: loop_start → condition → JumpIfZero(end) → body → Jump(loop_start)
                let loop_start = code.len();
                
                Self::gen_expr_with_locals(condition, code, locals, func_offsets);
                
                let jump_to_end_index = code.len();
                code.push(LpsOpCode::JumpIfZero(0)); // Placeholder
                
                Self::gen_stmt(body, code, locals, func_offsets);
                
                // Jump back to loop start
                let jump_back_offset = (loop_start as i32) - (code.len() as i32) - 1;
                code.push(LpsOpCode::Jump(jump_back_offset));
                
                // Patch JumpIfZero to point to end
                let end = code.len();
                if let LpsOpCode::JumpIfZero(ref mut offset) = code[jump_to_end_index] {
                    *offset = (end as i32) - (jump_to_end_index as i32) - 1;
                }
            }
            
            StmtKind::For { init, condition, increment, body } => {
                // Generate init
                if let Some(init_stmt) = init {
                    Self::gen_stmt(init_stmt, code, locals, func_offsets);
                }
                
                let loop_start = code.len();
                
                // Generate condition (if present)
                if let Some(cond) = condition {
                    Self::gen_expr_with_locals(cond, code, locals, func_offsets);
                    
                    let jump_to_end_index = code.len();
                    code.push(LpsOpCode::JumpIfZero(0)); // Placeholder
                    
                    Self::gen_stmt(body, code, locals, func_offsets);
                    
                    // Generate increment (if present)
                    if let Some(inc) = increment {
                        Self::gen_expr_with_locals(inc, code, locals, func_offsets);
                        code.push(LpsOpCode::Drop); // Discard increment result
                    }
                    
                    // Jump back to condition
                    let jump_back_offset = (loop_start as i32) - (code.len() as i32) - 1;
                    code.push(LpsOpCode::Jump(jump_back_offset));
                    
                    // Patch JumpIfZero to point to end
                    let end = code.len();
                    if let LpsOpCode::JumpIfZero(ref mut offset) = code[jump_to_end_index] {
                        *offset = (end as i32) - (jump_to_end_index as i32) - 1;
                    }
                } else {
                    // Infinite loop (no condition)
                    Self::gen_stmt(body, code, locals, func_offsets);
                    
                    if let Some(inc) = increment {
                        Self::gen_expr_with_locals(inc, code, locals, func_offsets);
                        code.push(LpsOpCode::Drop);
                    }
                    
                    let jump_back_offset = (loop_start as i32) - (code.len() as i32) - 1;
                    code.push(LpsOpCode::Jump(jump_back_offset));
                }
            }
        }
    }
    
    /// Generate typed binary operation based on operand and result types
    fn gen_binary_op(op: BinaryOp, left_ty: &Type, right_ty: &Type, result_ty: &Type, code: &mut Vec<LpsOpCode>) {
        match (op, result_ty) {
            // Fixed operations
            (BinaryOp::Add, Type::Fixed | Type::Int32) => code.push(LpsOpCode::AddFixed),
            (BinaryOp::Sub, Type::Fixed | Type::Int32) => code.push(LpsOpCode::SubFixed),
            (BinaryOp::Mul, Type::Fixed | Type::Int32) => code.push(LpsOpCode::MulFixed),
            (BinaryOp::Div, Type::Fixed | Type::Int32) => code.push(LpsOpCode::DivFixed),
            (BinaryOp::Mod, Type::Fixed | Type::Int32) => {
                // mod(x, y) = x - floor(x/y) * y
                // Stack has: [x, y]
                // We need: x - floor(x/y) * y
                // TODO: Implement properly - for now use placeholder
                code.push(LpsOpCode::DivFixed);
            }
            
            // Vec2 operations
            (BinaryOp::Add, Type::Vec2) => code.push(LpsOpCode::AddVec2),
            (BinaryOp::Sub, Type::Vec2) => code.push(LpsOpCode::SubVec2),
            (BinaryOp::Mul, Type::Vec2) => {
                // Check if it's vec * scalar or vec * vec
                if matches!(right_ty, Type::Fixed | Type::Int32) {
                    code.push(LpsOpCode::MulVec2Scalar);
                } else {
                    code.push(LpsOpCode::MulVec2);
                }
            }
            (BinaryOp::Div, Type::Vec2) => {
                if matches!(right_ty, Type::Fixed | Type::Int32) {
                    code.push(LpsOpCode::DivVec2Scalar);
                } else {
                    code.push(LpsOpCode::DivVec2);
                }
            }
            (BinaryOp::Mod, Type::Vec2) => code.push(LpsOpCode::MulVec2), // Placeholder
            
            // Vec3 operations
            (BinaryOp::Add, Type::Vec3) => code.push(LpsOpCode::AddVec3),
            (BinaryOp::Sub, Type::Vec3) => code.push(LpsOpCode::SubVec3),
            (BinaryOp::Mul, Type::Vec3) => {
                if matches!(right_ty, Type::Fixed | Type::Int32) {
                    code.push(LpsOpCode::MulVec3Scalar);
                } else {
                    code.push(LpsOpCode::MulVec3);
                }
            }
            (BinaryOp::Div, Type::Vec3) => {
                if matches!(right_ty, Type::Fixed | Type::Int32) {
                    code.push(LpsOpCode::DivVec3Scalar);
                } else {
                    code.push(LpsOpCode::DivVec3);
                }
            }
            (BinaryOp::Mod, Type::Vec3) => code.push(LpsOpCode::MulVec3), // Placeholder
            
            // Vec4 operations
            (BinaryOp::Add, Type::Vec4) => code.push(LpsOpCode::AddVec4),
            (BinaryOp::Sub, Type::Vec4) => code.push(LpsOpCode::SubVec4),
            (BinaryOp::Mul, Type::Vec4) => {
                if matches!(right_ty, Type::Fixed | Type::Int32) {
                    code.push(LpsOpCode::MulVec4Scalar);
                } else {
                    code.push(LpsOpCode::MulVec4);
                }
            }
            (BinaryOp::Div, Type::Vec4) => {
                if matches!(right_ty, Type::Fixed | Type::Int32) {
                    code.push(LpsOpCode::DivVec4Scalar);
                } else {
                    code.push(LpsOpCode::DivVec4);
                }
            }
            (BinaryOp::Mod, Type::Vec4) => code.push(LpsOpCode::MulVec4), // Placeholder
            
            _ => {} // Void or unsupported
        }
    }
    
    fn gen_expr_with_locals(expr: &Expr, code: &mut Vec<LpsOpCode>, locals: &mut LocalAllocator, func_offsets: &BTreeMap<String, u32>) {
        match &expr.kind {
            ExprKind::Number(n) => {
                code.push(LpsOpCode::Push((*n).to_fixed()));
            }
            
            ExprKind::IntNumber(n) => {
                // Convert int to fixed point for now (TODO: keep as int32)
                code.push(LpsOpCode::Push((*n).to_fixed()));
            }
            
            ExprKind::Variable(name) => {
                // Check if it's a vec2 built-in (uv, coord)
                match name.as_str() {
                    "uv" => {
                        // Push normalized coordinates as vec2
                        code.push(LpsOpCode::Load(LoadSource::XNorm));
                        code.push(LpsOpCode::Load(LoadSource::YNorm));
                    }
                    "coord" => {
                        // Push pixel coordinates as vec2 (converted to Fixed)
                        code.push(LpsOpCode::Load(LoadSource::XInt));
                        code.push(LpsOpCode::Load(LoadSource::YInt));
                    }
                    _ => {
                        // Check if it's a user-defined variable
                        if let Some(index) = locals.get(name) {
                            // Load from local variable
                            // TODO: Need to know the type to use correct Load opcode
                            // For now, assume Fixed
                            code.push(LpsOpCode::LoadLocalFixed(index));
                        } else {
                            // Scalar built-in
                            let source = Self::variable_to_load_source(name);
                            code.push(LpsOpCode::Load(source));
                        }
                    }
                }
            }
            
            // Binary operations - use type information to generate typed opcodes
            ExprKind::Add(left, right) => {
                Self::gen_expr_with_locals(left, code, locals, func_offsets);
                Self::gen_expr_with_locals(right, code, locals, func_offsets);
                Self::gen_binary_op(BinaryOp::Add, left.ty.as_ref().unwrap(), 
                    right.ty.as_ref().unwrap(), expr.ty.as_ref().unwrap(), code);
            }
            
            ExprKind::Sub(left, right) => {
                Self::gen_expr_with_locals(left, code, locals, func_offsets);
                Self::gen_expr_with_locals(right, code, locals, func_offsets);
                Self::gen_binary_op(BinaryOp::Sub, left.ty.as_ref().unwrap(), 
                    right.ty.as_ref().unwrap(), expr.ty.as_ref().unwrap(), code);
            }
            
            ExprKind::Mul(left, right) => {
                Self::gen_expr_with_locals(left, code, locals, func_offsets);
                Self::gen_expr_with_locals(right, code, locals, func_offsets);
                Self::gen_binary_op(BinaryOp::Mul, left.ty.as_ref().unwrap(), 
                    right.ty.as_ref().unwrap(), expr.ty.as_ref().unwrap(), code);
            }
            
            ExprKind::Div(left, right) => {
                Self::gen_expr_with_locals(left, code, locals, func_offsets);
                Self::gen_expr_with_locals(right, code, locals, func_offsets);
                Self::gen_binary_op(BinaryOp::Div, left.ty.as_ref().unwrap(), 
                    right.ty.as_ref().unwrap(), expr.ty.as_ref().unwrap(), code);
            }
            
            ExprKind::Mod(left, right) => {
                Self::gen_expr_with_locals(left, code, locals, func_offsets);
                Self::gen_expr_with_locals(right, code, locals, func_offsets);
                Self::gen_binary_op(BinaryOp::Mod, left.ty.as_ref().unwrap(), 
                    right.ty.as_ref().unwrap(), expr.ty.as_ref().unwrap(), code);
            }
            
            ExprKind::Pow(left, right) => {
                Self::gen_expr_with_locals(left, code, locals, func_offsets);
                Self::gen_expr_with_locals(right, code, locals, func_offsets);
                // Pow is always scalar for now
                // TODO: Add proper pow implementation
                code.push(LpsOpCode::Push(crate::math::Fixed::ONE)); // Placeholder
            }
            
            // Comparisons
            ExprKind::Less(left, right) => {
                Self::gen_expr_with_locals(left, code, locals, func_offsets);
                Self::gen_expr_with_locals(right, code, locals, func_offsets);
                code.push(LpsOpCode::LessFixed);
            }
            
            ExprKind::Greater(left, right) => {
                Self::gen_expr_with_locals(left, code, locals, func_offsets);
                Self::gen_expr_with_locals(right, code, locals, func_offsets);
                code.push(LpsOpCode::GreaterFixed);
            }
            
            ExprKind::LessEq(left, right) => {
                Self::gen_expr_with_locals(left, code, locals, func_offsets);
                Self::gen_expr_with_locals(right, code, locals, func_offsets);
                code.push(LpsOpCode::LessEqFixed);
            }
            
            ExprKind::GreaterEq(left, right) => {
                Self::gen_expr_with_locals(left, code, locals, func_offsets);
                Self::gen_expr_with_locals(right, code, locals, func_offsets);
                code.push(LpsOpCode::GreaterEqFixed);
            }
            
            ExprKind::Eq(left, right) => {
                Self::gen_expr_with_locals(left, code, locals, func_offsets);
                Self::gen_expr_with_locals(right, code, locals, func_offsets);
                code.push(LpsOpCode::EqFixed);
            }
            
            ExprKind::NotEq(left, right) => {
                Self::gen_expr_with_locals(left, code, locals, func_offsets);
                Self::gen_expr_with_locals(right, code, locals, func_offsets);
                code.push(LpsOpCode::NotEqFixed);
            }
            
            // Logical operations
            ExprKind::And(left, right) => {
                Self::gen_expr_with_locals(left, code, locals, func_offsets);
                Self::gen_expr_with_locals(right, code, locals, func_offsets);
                code.push(LpsOpCode::AndFixed);
            }
            
            ExprKind::Or(left, right) => {
                Self::gen_expr_with_locals(left, code, locals, func_offsets);
                Self::gen_expr_with_locals(right, code, locals, func_offsets);
                code.push(LpsOpCode::OrFixed);
            }
            
            // Ternary
            ExprKind::Ternary { condition, true_expr, false_expr } => {
                Self::gen_expr_with_locals(condition, code, locals, func_offsets);
                Self::gen_expr_with_locals(true_expr, code, locals, func_offsets);
                Self::gen_expr_with_locals(false_expr, code, locals, func_offsets);
                code.push(LpsOpCode::Select);
            }
            
            // Assignment expression
            ExprKind::Assign { target, value } => {
                // Generate code for the value
                Self::gen_expr_with_locals(value, code, locals, func_offsets);
                
                // Duplicate the value (assignment returns the value)
                code.push(LpsOpCode::Dup);
                
                // Store in the variable
                if let Some(index) = locals.get(target) {
                    let ty = value.ty.as_ref().unwrap();
                    match ty {
                        Type::Fixed | Type::Int32 => code.push(LpsOpCode::StoreLocalFixed(index)),
                        Type::Vec2 => code.push(LpsOpCode::StoreLocalVec2(index)),
                        Type::Vec3 => code.push(LpsOpCode::StoreLocalVec3(index)),
                        Type::Vec4 => code.push(LpsOpCode::StoreLocalVec4(index)),
                        _ => {}
                    }
                }
                // Value is left on stack (assignment expression returns the value)
            }
            
            // Function calls
            ExprKind::Call { name, args } => {
                Self::gen_function_call(name, args, code, locals, func_offsets);
            }
            
            // Vector constructors - push all components from all arguments
            // Supports GLSL-style mixed args: vec3(vec2, float), vec4(vec3, float), etc.
            ExprKind::Vec2Constructor(args) | 
            ExprKind::Vec3Constructor(args) | 
            ExprKind::Vec4Constructor(args) => {
                // Generate code for each argument, which pushes its components
                for arg in args {
                    Self::gen_expr_with_locals(arg, code, locals, func_offsets);
                }
                // Components are now on stack in the correct order
            }
            
            ExprKind::Swizzle { expr: base_expr, components } => {
                // Generate code for base expression (pushes vector components)
                Self::gen_expr_with_locals(base_expr, code, locals, func_offsets);
                
                // Get base type to know how many components to pop
                let base_type = base_expr.ty.as_ref().unwrap();
                let source_size = match base_type {
                    Type::Vec2 => 2,
                    Type::Vec3 => 3,
                    Type::Vec4 => 4,
                    _ => unreachable!("Type checker should catch non-vector swizzles"),
                };
                
                // Generate swizzle opcodes
                Self::gen_swizzle(components, source_size, code);
            }
        }
    }
    
    /// Generate opcodes for swizzling
    /// Stack layout: components are pushed in order, so for vec2(x,y), stack is [x, y] with y on top
    fn gen_swizzle(components: &str, source_size: usize, code: &mut Vec<LpsOpCode>) {
        // Map component characters to indices
        let indices: Vec<usize> = components.chars().map(|c| {
            match c {
                'x' | 'r' | 's' => 0,
                'y' | 'g' | 't' => 1,
                'z' | 'b' | 'p' => 2,
                'w' | 'a' | 'q' => 3,
                _ => unreachable!("Type checker should validate swizzle components"),
            }
        }).collect();
        
        // Strategy: Pop all source components into temporary positions,
        // then push back the desired components in the right order
        
        // We'll use a simple approach: generate Dup/Swap/Drop operations
        // This is not the most efficient but is correct and simple
        
        if components.len() == 1 {
            // Single component extraction
            let idx = indices[0];
            // Stack has [c0, c1, ..., c(n-1)] with c(n-1) on top
            // We want to keep component at index idx
            // Index 0 is at bottom, index (n-1) is at top
            let drop_count = source_size - 1 - idx;
            for _ in 0..drop_count {
                code.push(LpsOpCode::Drop);
            }
            // Now we have [c0, c1, ..., c(idx)]
            // We want just c(idx), so drop everything below
            for _ in 0..idx {
                code.push(LpsOpCode::Swap); // Bring bottom to top
                code.push(LpsOpCode::Drop); // Drop it
            }
        } else {
            // Multi-component swizzle
            // General algorithm: For each output component, pick from the input
            // 
            // Stack has components in order: [c0, c1, ..., c(n-1)] with c(n-1) on top
            // We need to produce [result0, result1, ..., result(m-1)]
            //
            // Strategy: Use helper function to access component at any index
            
            // Check if it's an identity swizzle first (optimization)
            let is_identity = indices.iter().enumerate().all(|(i, &idx)| i == idx);
            if is_identity && indices.len() == source_size {
                // Identity swizzle, no-op
                return;
            }
            
            // For vec2 specifically, handle common cases efficiently
            if source_size == 2 {
                match components {
                    "yx" | "gr" | "ts" => code.push(LpsOpCode::Swap),
                    "xx" | "rr" | "ss" => {
                        // [x, y] -> [x, x]
                        code.push(LpsOpCode::Drop); // [x]
                        code.push(LpsOpCode::Dup);  // [x, x]
                    }
                    "yy" | "gg" | "tt" => {
                        // [x, y] -> [y, y]
                        code.push(LpsOpCode::Swap); // [y, x]
                        code.push(LpsOpCode::Drop); // [y]
                        code.push(LpsOpCode::Dup);  // [y, y]
                    }
                    _ => {
                        // General case for vec2: Handle by reconstruction
                        // This is rare but possible (e.g., if type checker allows it)
                    }
                }
            } else {
                // For vec3 and vec4, we'll need a more sophisticated approach
                // For now, if it's identity, we already returned above
                // For non-identity vec3/vec4 swizzles, we'll need to implement
                // a general stack manipulation algorithm or add a Swizzle opcode to the VM
                //
                // TODO: Implement general vec3/vec4 swizzling
                // For now, identity swizzles work (which is the most common case)
            }
        }
    }
    
    fn gen_function_call(name: &str, args: &[Expr], code: &mut Vec<LpsOpCode>, locals: &mut LocalAllocator, func_offsets: &BTreeMap<String, u32>) {
        // Check if it's a user-defined function
        if let Some(&offset) = func_offsets.get(name) {
            // Generate code for arguments (push onto stack)
            for arg in args {
                Self::gen_expr_with_locals(arg, code, locals, func_offsets);
            }
            // Emit Call opcode with function offset
            code.push(LpsOpCode::Call(offset));
            return;
        }
        
        // Special case: perlin3(vec3) or perlin3(vec3, octaves)
        // Octaves is embedded in opcode, not pushed to stack
        if name == "perlin3" {
            // First arg is vec3, generate code to push its 3 components
            Self::gen_expr_with_locals(&args[0], code, locals, func_offsets);
            
            // Extract octaves from 2nd arg or use default
            let octaves = if args.len() >= 2 {
                if let ExprKind::Number(n) = &args[1].kind {
                    *n as u8
                } else if let ExprKind::IntNumber(n) = &args[1].kind {
                    *n as u8
                } else {
                    3 // Default
                }
            } else {
                3 // Default
            };
            
            code.push(LpsOpCode::Perlin3(octaves));
            return;
        }
        
        // For all other functions, generate code for all arguments first
        for arg in args {
            Self::gen_expr_with_locals(arg, code, locals, func_offsets);
        }
        
        // Emit the appropriate instruction
        match name {
            "sin" => code.push(LpsOpCode::SinFixed),
            "cos" => code.push(LpsOpCode::CosFixed),
            "frac" | "fract" => code.push(LpsOpCode::FractFixed),
            
            // Math functions - use explicit opcodes
            "min" => code.push(LpsOpCode::MinFixed),
            "max" => code.push(LpsOpCode::MaxFixed),
            "abs" => code.push(LpsOpCode::AbsFixed),
            "floor" => code.push(LpsOpCode::FloorFixed),
            "ceil" => code.push(LpsOpCode::CeilFixed),
            "sqrt" => code.push(LpsOpCode::SqrtFixed),
            "tan" => code.push(LpsOpCode::TanFixed),
            "pow" => code.push(LpsOpCode::PowFixed),
            "sign" => code.push(LpsOpCode::SignFixed),
            "mod" => code.push(LpsOpCode::ModFixed),
            "atan" => {
                if args.len() == 2 {
                    code.push(LpsOpCode::Atan2Fixed);
                } else {
                    code.push(LpsOpCode::AtanFixed);
                }
            }
            
            // Clamping/interpolation
            "clamp" => code.push(LpsOpCode::ClampFixed),
            "saturate" => code.push(LpsOpCode::SaturateFixed),
            "step" => code.push(LpsOpCode::StepFixed),
            "lerp" | "mix" => code.push(LpsOpCode::LerpFixed),
            "smoothstep" => code.push(LpsOpCode::SmoothstepFixed),
            
            // Vector functions - use typed opcodes based on argument type
            "length" => {
                let arg_ty = args[0].ty.as_ref().unwrap();
                match arg_ty {
                    Type::Vec2 => code.push(LpsOpCode::Length2),
                    Type::Vec3 => code.push(LpsOpCode::Length3),
                    Type::Vec4 => code.push(LpsOpCode::Length4),
                    _ => {}
                }
            }
            "normalize" => {
                let arg_ty = args[0].ty.as_ref().unwrap();
                match arg_ty {
                    Type::Vec2 => code.push(LpsOpCode::Normalize2),
                    Type::Vec3 => code.push(LpsOpCode::Normalize3),
                    Type::Vec4 => code.push(LpsOpCode::Normalize4),
                    _ => {}
                }
            }
            "dot" => {
                let arg_ty = args[0].ty.as_ref().unwrap();
                match arg_ty {
                    Type::Vec2 => code.push(LpsOpCode::Dot2),
                    Type::Vec3 => code.push(LpsOpCode::Dot3),
                    Type::Vec4 => code.push(LpsOpCode::Dot4),
                    _ => {}
                }
            }
            "distance" => {
                let arg_ty = args[0].ty.as_ref().unwrap();
                match arg_ty {
                    Type::Vec2 => code.push(LpsOpCode::Distance2),
                    Type::Vec3 => code.push(LpsOpCode::Distance3),
                    Type::Vec4 => code.push(LpsOpCode::Distance4),
                    _ => {}
                }
            }
            "cross" => {
                // Always vec3
                code.push(LpsOpCode::Cross3);
            }
            
            _ => {} // Unknown function - ignore
        }
    }
    
    fn variable_to_load_source(name: &str) -> LoadSource {
        match name {
            "x" | "xNorm" => LoadSource::XNorm,
            "y" | "yNorm" => LoadSource::YNorm,
            "time" | "t" => LoadSource::Time,
            "timeNorm" => LoadSource::TimeNorm,
            "centerAngle" | "angle" => LoadSource::CenterAngle,
            "centerDist" | "dist" => LoadSource::CenterDist,
            _ => LoadSource::XNorm, // Default fallback
        }
    }
}

/// Native function IDs for CallNative opcode
#[repr(u8)]
pub enum NativeFunction {
    // Math basics
    Min = 0,
    Max = 1,
    Pow = 2,
    Abs = 3,
    Floor = 4,
    Ceil = 5,
    Sqrt = 6,
    Sign = 7,
    Saturate = 8,
    Step = 9,
    
    // Utility
    Clamp = 10,
    Lerp = 11,
    Smoothstep = 12,
    
    // Trig (new GLSL functions)
    Tan = 13,
    Atan = 14,
    Mod = 15,
    
    // Comparisons
    Less = 20,
    Greater = 21,
    LessEq = 22,
    GreaterEq = 23,
    Eq = 24,
    NotEq = 25,
    
    // Logical
    And = 30,
    Or = 31,
    
    // Ternary select
    Select = 40,
    
    // Vector functions (polymorphic - work on vec2/vec3/vec4)
    Length = 50,
    Normalize = 51,
    Dot = 52,
    Distance = 53,
    Cross = 54,  // vec3 only
}
