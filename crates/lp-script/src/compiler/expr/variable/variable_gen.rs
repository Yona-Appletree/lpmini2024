/// Variable expression code generation
extern crate alloc;

use crate::compiler::codegen::CodeGenerator;
use crate::shared::Type;
use crate::vm::opcodes::LpsOpCode;
use crate::vm::opcodes::load::LoadSource;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_variable(&mut self, name: &str, var_type: &Type) {
        // Check if it's a vec2 built-in (uv, coord)
        match name {
            "uv" => {
                // Push normalized coordinates as vec2
                self.code.push(LpsOpCode::Load(LoadSource::XNorm));
                self.code.push(LpsOpCode::Load(LoadSource::YNorm));
            }
            "coord" => {
                // Push pixel coordinates as vec2 (converted to Fixed)
                self.code.push(LpsOpCode::Load(LoadSource::XInt));
                self.code.push(LpsOpCode::Load(LoadSource::YInt));
            }
            _ => {
                // Check if it's a user-defined variable
                if let Some(index) = self.locals.get(name) {
                    // Load from local variable using the correct opcode for the type
                    match var_type {
                        Type::Fixed | Type::Bool => {
                            self.code.push(LpsOpCode::LoadLocalFixed(index));
                        }
                        Type::Int32 => {
                            self.code.push(LpsOpCode::LoadLocalInt32(index));
                        }
                        Type::Vec2 => {
                            self.code.push(LpsOpCode::LoadLocalVec2(index));
                        }
                        Type::Vec3 => {
                            self.code.push(LpsOpCode::LoadLocalVec3(index));
                        }
                        Type::Vec4 => {
                            self.code.push(LpsOpCode::LoadLocalVec4(index));
                        }
                        _ => {
                            // Fallback for unsupported types
                            self.code.push(LpsOpCode::LoadLocalFixed(index));
                        }
                    }
                } else {
                    // Scalar built-in
                    let source = variable_to_load_source(name);
                    self.code.push(LpsOpCode::Load(source));
                }
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
