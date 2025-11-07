/// Control flow opcodes with error handling
extern crate alloc;
use alloc::vec::Vec;

use crate::vm::call_stack::CallStack;
use crate::vm::error::LpsVmError;
use crate::vm::local_stack::LocalStack;
use crate::vm::lps_program::LpsProgram;
use crate::vm::value_stack::ValueStack;
use crate::math::Fixed;

/// Action to take after executing Return opcode
#[derive(Debug)]
pub enum ReturnAction {
    /// Continue execution at the given PC (returning from function)
    /// (return_pc, return_fn_idx)
    Continue(usize, usize),
    /// Exit program with the given stack values (returning from main)
    Exit(Vec<Fixed>),
}

/// Execute Return: pop call frame and continue, or exit if in main
#[inline(always)]
pub fn exec_return(
    stack: &ValueStack,
    call_stack: &mut CallStack,
    locals: &mut LocalStack,
) -> Result<ReturnAction, LpsVmError> {
    if let Some((return_pc, return_fn_idx, locals_restore_sp)) = call_stack.pop_frame() {
        // Returning from a function call
        // Deallocate this function's locals
        locals.deallocate_to(locals_restore_sp);

        Ok(ReturnAction::Continue(return_pc, return_fn_idx))
    } else {
        // Exiting main - return all stack values as result
        let result: Vec<Fixed> = stack.raw_slice()[0..stack.sp()]
            .iter()
            .map(|&i| Fixed(i))
            .collect();
        Ok(ReturnAction::Exit(result))
    }
}

/// Execute Select (ternary): pop false_val, true_val, condition; push selected
#[inline(always)]
pub fn exec_select(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let (condition, true_val, false_val) = stack.pop3()?;

    stack.push_int32(if condition != 0 { true_val } else { false_val })?;

    Ok(())
}

/// Execute JumpIfZero: pop value, jump if zero
/// Returns Some(new_pc) if jump taken, None otherwise
#[inline(always)]
pub fn exec_jump_if_zero(
    stack: &mut ValueStack,
    pc: usize,
    offset: i32,
) -> Result<Option<usize>, LpsVmError> {
    let value = stack.pop_int32()?;

    if value == 0 {
        Ok(Some(((pc as i32) + offset + 1) as usize))
    } else {
        Ok(None)
    }
}

/// Execute JumpIfNonZero: pop value, jump if non-zero  
#[inline(always)]
pub fn exec_jump_if_nonzero(
    stack: &mut ValueStack,
    pc: usize,
    offset: i32,
) -> Result<Option<usize>, LpsVmError> {
    let value = stack.pop_int32()?;

    if value != 0 {
        Ok(Some(((pc as i32) + offset + 1) as usize))
    } else {
        Ok(None)
    }
}

/// Execute Jump: unconditional jump by offset
/// Returns the new PC value
#[inline(always)]
pub fn exec_jump(pc: usize, offset: i32, max_pc: usize) -> Result<usize, LpsVmError> {
    let new_pc = (pc as i32) + offset + 1;

    if new_pc < 0 || new_pc as usize >= max_pc {
        return Err(LpsVmError::ProgramCounterOutOfBounds {
            pc: new_pc as usize,
            max: max_pc,
        });
    }

    Ok(new_pc as usize)
}

/// Execute Call: call a function by index
///
/// Returns (new_pc, new_fn_idx) to switch execution to the called function
#[inline(always)]
pub fn exec_call(
    program: &LpsProgram,
    current_pc: usize,
    current_fn_idx: usize,
    target_fn_idx: usize,
    locals: &mut LocalStack,
    call_stack: &mut CallStack,
) -> Result<(usize, usize), LpsVmError> {
    // Get target function to allocate its locals
    let target_fn = program
        .function(target_fn_idx)
        .ok_or(LpsVmError::InvalidFunctionIndex)?;

    // Allocate locals for the called function BEFORE storing parameters
    let new_frame_base = locals.local_count();
    locals.allocate_locals(&target_fn.locals)?;

    // Push call frame
    let return_pc = current_pc + 1;
    let return_fn_idx = current_fn_idx;
    let current_locals_sp = new_frame_base; // Save where to restore

    call_stack.push_frame(
        return_pc,
        return_fn_idx,
        new_frame_base,
        current_locals_sp,
        target_fn_idx,
    )?;

    // Return new PC (0 = start of function) and new function index
    Ok((0, target_fn_idx))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::{Fixed, ToFixed};

    #[test]
    fn test_select_true() {
        let mut stack = ValueStack::new(64);

        // condition = 1 (true)
        stack.push_fixed(1.0f32.to_fixed()).unwrap();
        // true_val = 10
        stack.push_fixed(10.0f32.to_fixed()).unwrap();
        // false_val = 20
        stack.push_fixed(20.0f32.to_fixed()).unwrap();

        exec_select(&mut stack).unwrap();

        assert_eq!(stack.sp(), 1);
        assert_eq!(Fixed(stack.raw_slice()[0]).to_f32(), 10.0);
    }

    #[test]
    fn test_select_false() {
        let mut stack = ValueStack::new(64);

        // condition = 0 (false)
        stack.push_int32(0).unwrap();
        // true_val = 10
        stack.push_fixed(10.0f32.to_fixed()).unwrap();
        // false_val = 20
        stack.push_fixed(20.0f32.to_fixed()).unwrap();

        exec_select(&mut stack).unwrap();

        assert_eq!(stack.sp(), 1);
        assert_eq!(Fixed(stack.raw_slice()[0]).to_f32(), 20.0);
    }

    #[test]
    fn test_select_underflow() {
        let mut stack = ValueStack::new(64);
        // Only push 2 items, need 3
        stack.push_int32(1).unwrap();
        stack.push_int32(2).unwrap();

        let result = exec_select(&mut stack);
        assert!(matches!(
            result,
            Err(LpsVmError::StackUnderflow {
                required: 3,
                actual: 2
            })
        ));
    }

    #[test]
    fn test_jump() {
        // Test forward jump
        let result = exec_jump(10, 5, 100).unwrap();
        assert_eq!(result, 16); // pc + offset + 1 = 10 + 5 + 1 = 16

        // Test backward jump
        let result = exec_jump(20, -10, 100).unwrap();
        assert_eq!(result, 11); // 20 + (-10) + 1 = 11
    }

    #[test]
    fn test_jump_out_of_bounds() {
        // Jump beyond max_pc
        let result = exec_jump(90, 20, 100);
        assert!(matches!(
            result,
            Err(LpsVmError::ProgramCounterOutOfBounds { pc: 111, max: 100 })
        ));

        // Jump to negative
        let result = exec_jump(5, -10, 100);
        assert!(matches!(
            result,
            Err(LpsVmError::ProgramCounterOutOfBounds { .. })
        ));
    }

    #[test]
    fn test_jump_if_zero() {
        let mut stack = ValueStack::new(64);

        stack.push_int32(0).unwrap();

        let result = exec_jump_if_zero(&mut stack, 10, 5).unwrap();

        assert_eq!(stack.sp(), 0);
        assert_eq!(result, Some(16)); // pc + offset + 1 = 10 + 5 + 1 = 16
    }

    #[test]
    fn test_jump_if_zero_no_jump() {
        let mut stack = ValueStack::new(64);

        stack.push_fixed(1.0f32.to_fixed()).unwrap();

        let result = exec_jump_if_zero(&mut stack, 10, 5).unwrap();

        assert_eq!(stack.sp(), 0);
        assert_eq!(result, None);
    }

    #[test]
    fn test_return_from_function() {
        let stack = ValueStack::new(64);
        let mut call_stack = CallStack::new(64);
        let mut locals = LocalStack::new(1024);

        // Simulate a function call
        call_stack.push_frame(100, 0, 3, 3, 1).unwrap();

        // Execute return
        let result = exec_return(&stack, &mut call_stack, &mut locals).unwrap();

        // Should return to PC 100 and function 0
        match result {
            ReturnAction::Continue(pc, fn_idx) => {
                assert_eq!(pc, 100);
                assert_eq!(fn_idx, 0);
            }
            _ => panic!("Expected Continue action"),
        }

        // Call stack should be back at depth 0
        assert_eq!(call_stack.depth(), 0);
    }

    #[test]
    fn test_return_from_main() {
        let mut stack = ValueStack::new(64);
        let mut call_stack = CallStack::new(64);
        let mut locals = LocalStack::new(1024);

        // Push some values on the stack
        stack.push_fixed(1.5.to_fixed()).unwrap();
        stack.push_fixed(2.5.to_fixed()).unwrap();
        stack.push_fixed(3.5.to_fixed()).unwrap();

        // Execute return from main (depth 0)
        let result = exec_return(&stack, &mut call_stack, &mut locals).unwrap();

        // Should exit with stack values
        match result {
            ReturnAction::Exit(values) => {
                assert_eq!(values.len(), 3);
                assert_eq!(values[0].to_f32(), 1.5);
                assert_eq!(values[1].to_f32(), 2.5);
                assert_eq!(values[2].to_f32(), 3.5);
            }
            _ => panic!("Expected Exit action"),
        }
    }

    #[test]
    fn test_return_nested_calls() {
        let stack = ValueStack::new(64);
        let mut call_stack = CallStack::new(64);
        let mut locals = LocalStack::new(1024);

        // Simulate nested function calls
        call_stack.push_frame(100, 0, 3, 3, 1).unwrap();
        call_stack.push_frame(200, 1, 8, 8, 2).unwrap();
        call_stack.push_frame(300, 2, 13, 13, 3).unwrap();

        assert_eq!(call_stack.depth(), 3);

        // Return from innermost function
        let result = exec_return(&stack, &mut call_stack, &mut locals).unwrap();
        match result {
            ReturnAction::Continue(pc, fn_idx) => {
                assert_eq!(pc, 300);
                assert_eq!(fn_idx, 2);
            }
            _ => panic!("Expected Continue action"),
        }
        assert_eq!(call_stack.depth(), 2);

        // Return from middle function
        let result = exec_return(&stack, &mut call_stack, &mut locals).unwrap();
        match result {
            ReturnAction::Continue(pc, fn_idx) => {
                assert_eq!(pc, 200);
                assert_eq!(fn_idx, 1);
            }
            _ => panic!("Expected Continue action"),
        }
        assert_eq!(call_stack.depth(), 1);

        // Return from outer function
        let result = exec_return(&stack, &mut call_stack, &mut locals).unwrap();
        match result {
            ReturnAction::Continue(pc, fn_idx) => {
                assert_eq!(pc, 100);
                assert_eq!(fn_idx, 0);
            }
            _ => panic!("Expected Continue action"),
        }
        assert_eq!(call_stack.depth(), 0);

        // Return from main should exit
        let result = exec_return(&stack, &mut call_stack, &mut locals).unwrap();
        match result {
            ReturnAction::Exit(_) => {}
            _ => panic!("Expected Exit action"),
        }
    }

    #[test]
    fn test_call() {
        use crate::shared::Type;
        use crate::vm::lps_program::{FunctionDef, LocalVarDef};

        // Create a simple program with 2 functions
        let main_fn = FunctionDef::new("main".into(), Type::Void)
            .with_locals(vec![LocalVarDef::new("x".into(), Type::Fixed)]);

        let target_fn = FunctionDef::new("foo".into(), Type::Fixed).with_locals(vec![
            LocalVarDef::new("a".into(), Type::Fixed),
            LocalVarDef::new("b".into(), Type::Vec2),
        ]);

        let program = LpsProgram::new("test".into()).with_functions(vec![main_fn, target_fn]);

        let mut locals = LocalStack::new(1024);
        let mut call_stack = CallStack::new(64);

        // Allocate main's locals
        locals
            .allocate_locals(&program.function(0).unwrap().locals)
            .unwrap();
        assert_eq!(locals.local_count(), 1); // main has 1 local

        // Call function 1
        let (new_pc, new_fn_idx) = exec_call(
            &program,
            10, // current_pc
            0,  // current_fn_idx (main)
            1,  // target_fn_idx (foo)
            &mut locals,
            &mut call_stack,
        )
        .unwrap();

        assert_eq!(new_pc, 0); // Start at beginning of function
        assert_eq!(new_fn_idx, 1); // Switch to function 1
        assert_eq!(call_stack.depth(), 1); // One frame on call stack
        assert_eq!(locals.local_count(), 3); // main(1 local) + foo(2 locals) = 3 logical locals
    }

    #[test]
    fn test_call_invalid_function() {
        use crate::shared::Type;
        use crate::vm::lps_program::FunctionDef;

        let main_fn = FunctionDef::new("main".into(), Type::Void);
        let program = LpsProgram::new("test".into()).with_functions(vec![main_fn]);

        let mut locals = LocalStack::new(1024);
        let mut call_stack = CallStack::new(64);

        // Try to call non-existent function
        let result = exec_call(
            &program,
            10,
            0,
            99, // Invalid function index
            &mut locals,
            &mut call_stack,
        );

        assert!(matches!(result, Err(LpsVmError::InvalidFunctionIndex)));
    }
}
