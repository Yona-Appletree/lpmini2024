/// Control flow opcodes with error handling
extern crate alloc;
use alloc::vec::Vec;

use crate::lpscript::vm::call_stack::CallStack;
use crate::lpscript::vm::error::RuntimeError;
use crate::lpscript::vm::vm_stack::Stack;
use crate::math::Fixed;

/// Action to take after executing Return opcode
#[derive(Debug)]
pub enum ReturnAction {
    /// Continue execution at the given PC (returning from function)
    Continue(usize),
    /// Exit program with the given stack values (returning from main)
    Exit(Vec<Fixed>),
}

/// Execute Return: pop call frame and continue, or exit if in main
#[inline(always)]
pub fn exec_return(
    stack: &Stack,
    call_stack: &mut CallStack,
) -> Result<ReturnAction, RuntimeError> {
    if let Some((return_pc, _return_fn_idx, _locals_restore_sp)) = call_stack.pop_frame() {
        // Returning from a function call
        // TODO: Handle locals_restore_sp to deallocate locals
        Ok(ReturnAction::Continue(return_pc))
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
pub fn exec_select(stack: &mut Stack) -> Result<(), RuntimeError> {
    let (condition, true_val, false_val) = stack.pop3()?;

    stack.push_int32(if condition != 0 { true_val } else { false_val })?;

    Ok(())
}

/// Execute JumpIfZero: pop value, jump if zero
/// Returns Some(new_pc) if jump taken, None otherwise
#[inline(always)]
pub fn exec_jump_if_zero(
    stack: &mut Stack,
    pc: usize,
    offset: i32,
) -> Result<Option<usize>, RuntimeError> {
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
    stack: &mut Stack,
    pc: usize,
    offset: i32,
) -> Result<Option<usize>, RuntimeError> {
    let value = stack.pop_int32()?;

    if value != 0 {
        Ok(Some(((pc as i32) + offset + 1) as usize))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::{Fixed, ToFixed};

    #[test]
    fn test_select_true() {
        let mut stack = Stack::new(64);

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
        let mut stack = Stack::new(64);

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
        let mut stack = Stack::new(64);
        // Only push 2 items, need 3
        stack.push_int32(1).unwrap();
        stack.push_int32(2).unwrap();

        let result = exec_select(&mut stack);
        assert!(matches!(
            result,
            Err(RuntimeError::StackUnderflow {
                required: 3,
                actual: 2
            })
        ));
    }

    #[test]
    fn test_jump_if_zero() {
        let mut stack = Stack::new(64);

        stack.push_int32(0).unwrap();

        let result = exec_jump_if_zero(&mut stack, 10, 5).unwrap();

        assert_eq!(stack.sp(), 0);
        assert_eq!(result, Some(16)); // pc + offset + 1 = 10 + 5 + 1 = 16
    }

    #[test]
    fn test_jump_if_zero_no_jump() {
        let mut stack = Stack::new(64);

        stack.push_fixed(1.0f32.to_fixed()).unwrap();

        let result = exec_jump_if_zero(&mut stack, 10, 5).unwrap();

        assert_eq!(stack.sp(), 0);
        assert_eq!(result, None);
    }

    #[test]
    fn test_return_from_function() {
        let stack = Stack::new(64);
        let mut call_stack = CallStack::new(64);

        // Simulate a function call
        call_stack.push_frame(100, 0, 3, 3, 1).unwrap();

        // Execute return
        let result = exec_return(&stack, &mut call_stack).unwrap();

        // Should return to PC 100
        match result {
            ReturnAction::Continue(pc) => {
                assert_eq!(pc, 100);
            }
            _ => panic!("Expected Continue action"),
        }

        // Call stack should be back at depth 0
        assert_eq!(call_stack.depth(), 0);
    }

    #[test]
    fn test_return_from_main() {
        let mut stack = Stack::new(64);
        let mut call_stack = CallStack::new(64);

        // Push some values on the stack
        stack.push_fixed(1.5.to_fixed()).unwrap();
        stack.push_fixed(2.5.to_fixed()).unwrap();
        stack.push_fixed(3.5.to_fixed()).unwrap();

        // Execute return from main (depth 0)
        let result = exec_return(&stack, &mut call_stack).unwrap();

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
        let stack = Stack::new(64);
        let mut call_stack = CallStack::new(64);

        // Simulate nested function calls
        call_stack.push_frame(100, 0, 3, 3, 1).unwrap();
        call_stack.push_frame(200, 1, 8, 8, 2).unwrap();
        call_stack.push_frame(300, 2, 13, 13, 3).unwrap();

        assert_eq!(call_stack.depth(), 3);

        // Return from innermost function
        let result = exec_return(&stack, &mut call_stack).unwrap();
        match result {
            ReturnAction::Continue(pc) => assert_eq!(pc, 300),
            _ => panic!("Expected Continue action"),
        }
        assert_eq!(call_stack.depth(), 2);

        // Return from middle function
        let result = exec_return(&stack, &mut call_stack).unwrap();
        match result {
            ReturnAction::Continue(pc) => assert_eq!(pc, 200),
            _ => panic!("Expected Continue action"),
        }
        assert_eq!(call_stack.depth(), 1);

        // Return from outer function
        let result = exec_return(&stack, &mut call_stack).unwrap();
        match result {
            ReturnAction::Continue(pc) => assert_eq!(pc, 100),
            _ => panic!("Expected Continue action"),
        }
        assert_eq!(call_stack.depth(), 0);

        // Return from main should exit
        let result = exec_return(&stack, &mut call_stack).unwrap();
        match result {
            ReturnAction::Exit(_) => {}
            _ => panic!("Expected Exit action"),
        }
    }
}
