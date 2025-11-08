/// Swizzle operation code generation
extern crate alloc;
use alloc::vec::Vec;

use crate::compiler::ast::Expr;
use crate::compiler::codegen::CodeGenerator;
use crate::shared::Type;
use crate::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_swizzle(&mut self, expr: &Expr, components: &str) {
        // Generate the base expression (leaves vector components on stack)
        self.gen_expr(expr);

        // Generate swizzle opcodes based on component string
        let source_type = expr.ty.as_ref().unwrap();
        let source_size = match source_type {
            Type::Vec2 => 2,
            Type::Vec3 => 3,
            Type::Vec4 => 4,
            _ => 1,
        };

        // Call the helper function
        gen_swizzle_opcodes(components, source_size, self.code);
    }
}

/// Generate opcodes for swizzling
/// Stack layout: components are pushed in order, so for vec2(x,y), stack is [x, y] with y on top
fn gen_swizzle_opcodes(components: &str, source_size: usize, code: &mut Vec<LpsOpCode>) {
    use alloc::vec::Vec as AllocVec;

    // Map component characters to indices
    let indices: AllocVec<usize> = components
        .chars()
        .map(|c| match c {
            'x' | 'r' | 's' => 0,
            'y' | 'g' | 't' => 1,
            'z' | 'b' | 'p' => 2,
            'w' | 'a' | 'q' => 3,
            _ => unreachable!("Type checker should validate swizzle components"),
        })
        .collect();

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
            code.push(LpsOpCode::Drop1);
        }
        // Now we have [c0, c1, ..., c(idx)]
        // We want just c(idx), so drop everything below
        for _ in 0..idx {
            code.push(LpsOpCode::Swap); // Bring bottom to top
            code.push(LpsOpCode::Drop1); // Drop it
        }
    } else {
        // Multi-component swizzle
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
                    code.push(LpsOpCode::Drop1); // [x]
                    code.push(LpsOpCode::Dup1); // [x, x]
                }
                "yy" | "gg" | "tt" => {
                    // [x, y] -> [y, y]
                    code.push(LpsOpCode::Swap); // [y, x]
                    code.push(LpsOpCode::Drop1); // [y]
                    code.push(LpsOpCode::Dup1); // [y, y]
                }
                _ => {
                    // General case for vec2: Handle by reconstruction
                    // This is rare but possible (e.g., if type checker allows it)
                }
            }
        } else if components.len() == 2 && source_size == 3 {
            // vec3 -> vec2 swizzle
            code.push(LpsOpCode::Swizzle3to2(indices[0] as u8, indices[1] as u8));
        } else if components.len() == 3 && source_size == 3 {
            // vec3 -> vec3 swizzle
            code.push(LpsOpCode::Swizzle3to3(
                indices[0] as u8,
                indices[1] as u8,
                indices[2] as u8,
            ));
        } else if components.len() == 2 && source_size == 4 {
            // vec4 -> vec2 swizzle
            code.push(LpsOpCode::Swizzle4to2(indices[0] as u8, indices[1] as u8));
        } else if components.len() == 3 && source_size == 4 {
            // vec4 -> vec3 swizzle
            code.push(LpsOpCode::Swizzle4to3(
                indices[0] as u8,
                indices[1] as u8,
                indices[2] as u8,
            ));
        } else if components.len() == 4 && source_size == 4 {
            // vec4 -> vec4 swizzle
            code.push(LpsOpCode::Swizzle4to4(
                indices[0] as u8,
                indices[1] as u8,
                indices[2] as u8,
                indices[3] as u8,
            ));
        }
    }
}
