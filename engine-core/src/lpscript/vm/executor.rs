/// LPS VM executor - runs compiled programs
extern crate alloc;
use alloc::vec::Vec;
use alloc::string::ToString;

use crate::math::{Fixed, Vec2, Vec3, Vec4};
use crate::lpscript::error::{RuntimeError, RuntimeErrorWithContext};
use crate::lpscript::vm::opcodes::LpsOpCode;
use crate::test_engine::LoadSource;
use super::program::LpsProgram;
use super::locals::LocalType;

/// Call frame for function calls
#[derive(Debug, Clone, Copy)]
struct CallFrame {
    return_pc: usize,
    // TODO: Could add frame pointer for local variables if needed
}

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
    call_stack: Vec<CallFrame>,
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
            call_stack: Vec::new(),
        })
    }
    
    /// Execute the program for a single pixel
    pub fn run(&mut self, x: Fixed, y: Fixed, time: Fixed) -> Result<Fixed, RuntimeErrorWithContext> {
        self.sp = 0;
        self.pc = 0;
        self.call_stack.clear();
        
        // Store built-in values for Load operations
        let x_norm = x;
        let y_norm = y;
        let x_int = Fixed::from_i32(x.to_i32()); // Convert normalized to int coords
        let y_int = Fixed::from_i32(y.to_i32());
        
        loop {
            if self.pc >= self.program.opcodes.len() {
                return Err(RuntimeErrorWithContext {
                    error: RuntimeError::ProgramCounterOutOfBounds {
                        pc: self.pc,
                        max: self.program.opcodes.len(),
                    },
                    pc: self.pc,
                    opcode: "EOF",
                });
            }
            
            let opcode = &self.program.opcodes[self.pc];
            
            match opcode {
                // === Stack Operations ===
                LpsOpCode::Push(val) => {
                    self.push(*val)?;
                    self.pc += 1;
                }
                
                LpsOpCode::PushInt32(val) => {
                    self.push(Fixed::from_i32(*val))?;
                    self.pc += 1;
                }
                
                LpsOpCode::Dup => {
                    let val = self.peek()?;
                    self.push(val)?;
                    self.pc += 1;
                }
                
                LpsOpCode::Drop => {
                    self.pop()?;
                    self.pc += 1;
                }
                
                LpsOpCode::Swap => {
                    let a = self.pop()?;
                    let b = self.pop()?;
                    self.push(a)?;
                    self.push(b)?;
                    self.pc += 1;
                }
                
                // === Control Flow ===
                LpsOpCode::Jump(offset) => {
                    let new_pc = (self.pc as i32) + offset + 1;
                    if new_pc < 0 || new_pc as usize >= self.program.opcodes.len() {
                        return Err(self.runtime_error(RuntimeError::ProgramCounterOutOfBounds {
                            pc: new_pc as usize,
                            max: self.program.opcodes.len(),
                        }));
                    }
                    self.pc = new_pc as usize;
                }
                
                LpsOpCode::JumpIfZero(offset) => {
                    let offset = *offset; // Clone before mutable borrow
                    let cond = self.pop()?;
                    if cond.is_zero() {
                        let new_pc = (self.pc as i32) + offset + 1;
                        if new_pc < 0 || new_pc as usize >= self.program.opcodes.len() {
                            return Err(self.runtime_error(RuntimeError::ProgramCounterOutOfBounds {
                                pc: new_pc as usize,
                                max: self.program.opcodes.len(),
                            }));
                        }
                        self.pc = new_pc as usize;
                    } else {
                        self.pc += 1;
                    }
                }
                
                LpsOpCode::Call(offset) => {
                    // Push return address onto call stack
                    self.call_stack.push(CallFrame {
                        return_pc: self.pc + 1,
                    });
                    // Jump to function
                    self.pc = *offset as usize;
                }
                
                LpsOpCode::Return => {
                    // Check if we're returning from a function or exiting main
                    if let Some(frame) = self.call_stack.pop() {
                        // Return from function - jump back to caller
                        self.pc = frame.return_pc;
                    } else {
                        // Exiting main - return top of stack as result
                        return Ok(self.pop()?);
                    }
                }
                
                // === Load Built-in Variables ===
                LpsOpCode::Load(source) => {
                    let val = match source {
                        LoadSource::XNorm => x_norm,
                        LoadSource::YNorm => y_norm,
                        LoadSource::XInt => x_int,
                        LoadSource::YInt => y_int,
                        LoadSource::Time => time,
                        LoadSource::TimeNorm => time, // TODO: normalize if needed
                        _ => Fixed::ZERO,
                    };
                    self.push(val)?;
                    self.pc += 1;
                }
                
                // === Local Variables ===
                LpsOpCode::LoadLocalFixed(idx) => {
                    if (*idx as usize) >= self.locals.len() {
                        return Err(self.runtime_error(RuntimeError::LocalOutOfBounds {
                            local_idx: *idx as usize,
                            max: self.locals.len(),
                        }));
                    }
                    match self.locals[*idx as usize] {
                        LocalType::Fixed(val) => self.push(val)?,
                        _ => return Err(self.runtime_error(RuntimeError::TypeMismatch)),
                    }
                    self.pc += 1;
                }
                
                LpsOpCode::StoreLocalFixed(idx) => {
                    let idx = *idx; // Clone before mutable borrow
                    let val = self.pop()?;
                    if (idx as usize) >= self.locals.len() {
                        // Auto-grow locals array if needed
                        self.locals.resize(idx as usize + 1, LocalType::Fixed(Fixed::ZERO));
                    }
                    self.locals[idx as usize] = LocalType::Fixed(val);
                    self.pc += 1;
                }
                
                LpsOpCode::LoadLocalVec2(idx) => {
                    if (*idx as usize) >= self.locals.len() {
                        return Err(self.runtime_error(RuntimeError::LocalOutOfBounds {
                            local_idx: *idx as usize,
                            max: self.locals.len(),
                        }));
                    }
                    match self.locals[*idx as usize] {
                        LocalType::Vec2(x, y) => {
                            self.push(x)?;
                            self.push(y)?;
                        }
                        _ => return Err(self.runtime_error(RuntimeError::TypeMismatch)),
                    }
                    self.pc += 1;
                }
                
                LpsOpCode::StoreLocalVec2(idx) => {
                    let idx = *idx; // Clone before mutable borrow
                    let y = self.pop()?;
                    let x = self.pop()?;
                    if (idx as usize) >= self.locals.len() {
                        self.locals.resize(idx as usize + 1, LocalType::Fixed(Fixed::ZERO));
                    }
                    self.locals[idx as usize] = LocalType::Vec2(x, y);
                    self.pc += 1;
                }
                
                // === Fixed-point Arithmetic ===
                LpsOpCode::AddFixed => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(a + b)?;
                    self.pc += 1;
                }
                
                LpsOpCode::SubFixed => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(a - b)?;
                    self.pc += 1;
                }
                
                LpsOpCode::MulFixed => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(a * b)?;
                    self.pc += 1;
                }
                
                LpsOpCode::DivFixed => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(a / b)?;
                    self.pc += 1;
                }
                
                LpsOpCode::NegFixed => {
                    let a = self.pop()?;
                    self.push(-a)?;
                    self.pc += 1;
                }
                
                // === Comparisons ===
                LpsOpCode::GreaterFixed => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(if a > b { Fixed::ONE } else { Fixed::ZERO })?;
                    self.pc += 1;
                }
                
                LpsOpCode::LessFixed => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(if a < b { Fixed::ONE } else { Fixed::ZERO })?;
                    self.pc += 1;
                }
                
                LpsOpCode::GreaterEqFixed => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(if a >= b { Fixed::ONE } else { Fixed::ZERO })?;
                    self.pc += 1;
                }
                
                LpsOpCode::LessEqFixed => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(if a <= b { Fixed::ONE } else { Fixed::ZERO })?;
                    self.pc += 1;
                }
                
                LpsOpCode::EqFixed => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(if a == b { Fixed::ONE } else { Fixed::ZERO })?;
                    self.pc += 1;
                }
                
                LpsOpCode::NotEqFixed => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(if a != b { Fixed::ONE } else { Fixed::ZERO })?;
                    self.pc += 1;
                }
                
                // === Ternary Select ===
                LpsOpCode::Select => {
                    let false_val = self.pop()?;
                    let true_val = self.pop()?;
                    let cond = self.pop()?;
                    self.push(if !cond.is_zero() { true_val } else { false_val })?;
                    self.pc += 1;
                }
                
                // === Trigonometry ===
                LpsOpCode::SinFixed => {
                    let a = self.pop()?;
                    self.push(crate::math::sin(a))?;
                    self.pc += 1;
                }
                
                LpsOpCode::CosFixed => {
                    let a = self.pop()?;
                    self.push(crate::math::cos(a))?;
                    self.pc += 1;
                }
                
                // === Vec2 Operations ===
                LpsOpCode::AddVec2 => {
                    let b_y = self.pop()?;
                    let b_x = self.pop()?;
                    let a_y = self.pop()?;
                    let a_x = self.pop()?;
                    self.push(a_x + b_x)?;
                    self.push(a_y + b_y)?;
                    self.pc += 1;
                }
                
                LpsOpCode::SubVec2 => {
                    let b_y = self.pop()?;
                    let b_x = self.pop()?;
                    let a_y = self.pop()?;
                    let a_x = self.pop()?;
                    self.push(a_x - b_x)?;
                    self.push(a_y - b_y)?;
                    self.pc += 1;
                }
                
                LpsOpCode::MulVec2Scalar => {
                    let scalar = self.pop()?;
                    let y = self.pop()?;
                    let x = self.pop()?;
                    self.push(x * scalar)?;
                    self.push(y * scalar)?;
                    self.pc += 1;
                }
                
                // For now, handle other opcodes with a placeholder
                _ => {
                    // TODO: Implement remaining opcodes
                    return Err(self.runtime_error(RuntimeError::UnsupportedOpCode));
                }
            }
        }
    }
    
    // Helper methods for stack management
    fn push(&mut self, val: Fixed) -> Result<(), RuntimeErrorWithContext> {
        if self.sp >= self.stack.len() {
            return Err(self.runtime_error(RuntimeError::StackOverflow { sp: self.sp }));
        }
        self.stack[self.sp] = val.0;
        self.sp += 1;
        Ok(())
    }
    
    fn pop(&mut self) -> Result<Fixed, RuntimeErrorWithContext> {
        if self.sp == 0 {
            return Err(self.runtime_error(RuntimeError::StackUnderflow { required: 1, actual: 0 }));
        }
        self.sp -= 1;
        Ok(Fixed(self.stack[self.sp]))
    }
    
    fn peek(&self) -> Result<Fixed, RuntimeErrorWithContext> {
        if self.sp == 0 {
            return Err(RuntimeErrorWithContext {
                error: RuntimeError::StackUnderflow { required: 1, actual: 0 },
                pc: self.pc,
                opcode: "peek",
            });
        }
        Ok(Fixed(self.stack[self.sp - 1]))
    }
    
    fn runtime_error(&self, error: RuntimeError) -> RuntimeErrorWithContext {
        RuntimeErrorWithContext {
            error,
            pc: self.pc,
            opcode: "opcode", // TODO: Get actual opcode name
        }
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
    
    #[test]
    fn test_vm_simple_expression() {
        use crate::math::ToFixed;
        
        let program = parse_expr("1.0 + 2.0");
        let mut vm = LpsVm::new(program, vec![]).unwrap();
        
        let result = vm.run(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO).unwrap();
        assert_eq!(result.to_f32(), 3.0);
    }
    
    #[test]
    fn test_vm_with_builtin_vars() {
        use crate::math::ToFixed;
        
        let program = parse_expr("uv.x + uv.y");
        let mut vm = LpsVm::new(program, vec![]).unwrap();
        
        let result = vm.run(0.5.to_fixed(), 0.3.to_fixed(), Fixed::ZERO).unwrap();
        assert!((result.to_f32() - 0.8).abs() < 0.01); // Account for fixed-point precision
    }
    
    #[test]
    fn test_vm_comparisons() {
        let program = parse_expr("5.0 > 3.0");
        let mut vm = LpsVm::new(program, vec![]).unwrap();
        
        let result = vm.run(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO).unwrap();
        assert_eq!(result, Fixed::ONE); // TRUE
    }
    
    #[test]
    fn test_vm_user_function() {
        use crate::lpscript::parse_script;
        
        let script = "
            float double(float x) {
                return x * 2.0;
            }
            
            return double(5.0);
        ";
        let program = parse_script(script);
        let mut vm = LpsVm::new(program, vec![]).unwrap();
        
        let result = vm.run(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO).unwrap();
        assert_eq!(result.to_f32(), 10.0);
    }
    
    #[test]
    fn test_vm_function_with_multiple_params() {
        use crate::lpscript::parse_script;
        
        let script = "
            float add(float a, float b) {
                return a + b;
            }
            
            return add(3.0, 7.0);
        ";
        let program = parse_script(script);
        let mut vm = LpsVm::new(program, vec![]).unwrap();
        
        let result = vm.run(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO).unwrap();
        assert_eq!(result.to_f32(), 10.0);
    }
}

