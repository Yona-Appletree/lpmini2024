/// Vector constructor code generation
extern crate alloc;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::string::String;

use crate::lpscript::ast::Expr;
use crate::lpscript::vm::opcodes::LpsOpCode;
use super::super::local_allocator::LocalAllocator;

pub fn gen_vec_constructor(
    args: &[Expr],
    code: &mut Vec<LpsOpCode>,
    locals: &mut LocalAllocator,
    func_offsets: &BTreeMap<String, u32>,
    gen_expr: impl Fn(&Expr, &mut Vec<LpsOpCode>, &mut LocalAllocator, &BTreeMap<String, u32>) + Copy,
) {
    // Generate code for each argument, which pushes its components
    // Supports GLSL-style mixed args: vec3(vec2, float), vec4(vec3, float), etc.
    for arg in args {
        gen_expr(arg, code, locals, func_offsets);
    }
    // Components are now on stack in the correct order
}

