/// LPS VM executor - runs compiled programs
extern crate alloc;
use alloc::vec::Vec;

use crate::math::Fixed;
use crate::lpscript::error::{RuntimeError, RuntimeErrorWithContext};
use super::program::LpsProgram;
use super::locals::LocalType;

/// LightPlayer Script Virtual Machine
/// 
/// Executes compiled LPS programs. Designed to be reusable - create once,
/// then call run() for each pixel.
pub struct LpsVm {
    pub program: LpsProgram,
    stack: [i32; 64],
    sp: usize,
    #[allow(dead_code)]
    pc: usize,
    #[allow(dead_code)]
    locals: Vec<LocalType>,
}

impl LpsVm {
    /// Create a new VM from a program with input locals
    pub fn new(program: LpsProgram, inputs: Vec<(usize, LocalType)>) -> Result<Self, RuntimeError> {
        let mut locals = Vec::new();
        locals.resize(program.locals.len(), LocalType::Fixed(Fixed::ZERO));
        
        // Set input locals
        for (idx, local) in inputs {
            if idx >= locals.len() {
                return Err(RuntimeError::LocalOutOfBounds {
                    local_idx: idx,
                    max: locals.len(),
                });
            }
            locals[idx] = local;
        }
        
        Ok(LpsVm {
            program,
            stack: [0; 64],
            sp: 0,
            pc: 0,
            locals,
        })
    }
    
    /// Execute the program for a single pixel
    /// 
    /// Currently delegates to the legacy VM in test_engine.
    /// Full implementation with new typed opcodes is planned.
    pub fn run(&mut self, _x: Fixed, _y: Fixed, _time: Fixed) -> Result<Fixed, RuntimeErrorWithContext> {
        // TODO: Implement full VM execution with new typed opcodes
        // For now, this is a placeholder that demonstrates the API
        // The legacy execute_program in test_engine handles actual execution
        
        Ok(Fixed::ZERO)
    }
    
    /// Format a runtime error with full context
    pub fn format_error(&self, error: &RuntimeErrorWithContext) -> alloc::string::String {
        use alloc::format;
        
        let mut output = format!("{}\n", error);
        output.push_str(&format!("  at PC {} ({})\n", error.pc, error.opcode));
        output.push_str(&format!("  stack pointer: {}\n", self.sp));
        
        // Show top of stack
        if self.sp > 0 {
            output.push_str("  stack (top 5): [");
            let start = if self.sp > 5 { self.sp - 5 } else { 0 };
            for i in start..self.sp {
                if i > start {
                    output.push_str(", ");
                }
                output.push_str(&format!("{}", Fixed(self.stack[i]).to_f32()));
            }
            output.push_str("]\n");
        }
        
        // Show source if available
        if let Some(ref source) = self.program.source {
            if let Some(ref source_map) = self.program.source_map {
                if error.pc < source_map.len() {
                    let span = source_map[error.pc];
                    output.push_str(&format!("  source: {}\n", 
                        &source[span.start..span.end.min(source.len())]));
                }
            }
        }
        
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lpscript::parse_expr;
    
    #[test]
    fn test_vm_creation() {
        let program = parse_expr("1.0 + 2.0");
        let vm = LpsVm::new(program, vec![]).unwrap();
        
        // Verify VM can be created
        assert!(vm.program.opcodes.len() > 0);
    }
    
    #[test]
    fn test_vm_with_locals() {
        use crate::math::ToFixed;
        
        let mut program = parse_expr("xNorm");
        program.locals.push(crate::lpscript::LocalDef::new(
            "test".into(),
            LocalType::Fixed(1.0.to_fixed()),
            crate::lpscript::LocalAccess::Scratch,
        ));
        
        let vm = LpsVm::new(program, vec![]).unwrap();
        assert_eq!(vm.locals.len(), 1);
    }
}

