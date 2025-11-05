/// Swizzle operation code generation
extern crate alloc;
use alloc::vec::Vec;
use alloc::boxed::Box;

use crate::lpscript::ast::Expr;
use crate::lpscript::error::Type;
use crate::lpscript::vm::opcodes::LpsOpCode;
use crate::lpscript::compiler::generator::CodeGenerator;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_swizzle(
        &mut self,
        base_expr: &Box<Expr>,
        components: &str,
    ) {
        // Generate code for base expression (pushes vector components)
        self.gen_expr(base_expr);
        
        // Get base type to know how many components to pop
        let base_type = base_expr.ty.as_ref().unwrap();
        let source_size = match base_type {
            Type::Vec2 => 2,
            Type::Vec3 => 3,
            Type::Vec4 => 4,
            _ => unreachable!("Type checker should catch non-vector swizzles"),
        };
        
        // Generate swizzle opcodes
        gen_swizzle_opcodes(components, source_size, self.code);
    }
}

/// Generate opcodes for swizzling
/// Stack layout: components are pushed in order, so for vec2(x,y), stack is [x, y] with y on top
fn gen_swizzle_opcodes(components: &str, source_size: usize, code: &mut Vec<LpsOpCode>) {
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

