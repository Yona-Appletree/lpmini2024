/// Variable expression code generation
extern crate alloc;
use alloc::vec::Vec;

use crate::lpscript::vm::opcodes::LpsOpCode;
use crate::test_engine::LoadSource;
use super::super::local_allocator::LocalAllocator;

pub fn gen_variable(name: &str, code: &mut Vec<LpsOpCode>, locals: &LocalAllocator) {
    // Check if it's a vec2 built-in (uv, coord)
    match name {
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
                let source = variable_to_load_source(name);
                code.push(LpsOpCode::Load(source));
            }
        }
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

