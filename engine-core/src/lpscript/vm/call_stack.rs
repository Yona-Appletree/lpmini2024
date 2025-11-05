/// Call stack management for LPS VM function calls
extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

use super::error::RuntimeError;

/// Call frame for function calls
#[derive(Debug, Clone, Copy)]
pub struct CallFrame {
    pub return_pc: usize,
    pub frame_base: usize, // Base index in locals array for this frame
}

/// Call stack for managing function call frames
///
/// Pre-allocates frames to avoid runtime allocations during execution.
/// Tracks current depth and provides frame-based local variable management.
pub struct CallStack {
    frames: Vec<CallFrame>,
    depth: usize,
    max_depth: usize,
    frame_base: usize,  // Current frame's base index in locals array
    locals_sp: usize,   // Next free slot in locals array
    frame_size: usize,  // Number of locals per frame (typically 32)
}

impl CallStack {
    /// Create a new call stack
    ///
    /// # Arguments
    /// * `max_depth` - Maximum call depth (e.g., 64)
    /// * `frame_size` - Number of locals per frame (e.g., 32)
    pub fn new(max_depth: usize, frame_size: usize) -> Self {
        CallStack {
            frames: vec![
                CallFrame {
                    return_pc: 0,
                    frame_base: 0
                };
                max_depth
            ],
            depth: 0,
            max_depth,
            frame_base: 0,
            locals_sp: frame_size, // Main frame uses 0..(frame_size-1), next starts at frame_size
            frame_size,
        }
    }

    /// Reset the call stack for a new execution
    #[inline(always)]
    pub fn reset(&mut self) {
        self.depth = 0;
        self.frame_base = 0;
        self.locals_sp = self.frame_size;
    }

    /// Get current call depth
    #[inline(always)]
    pub fn depth(&self) -> usize {
        self.depth
    }

    /// Get current frame base (for local variable indexing)
    #[inline(always)]
    pub fn frame_base(&self) -> usize {
        self.frame_base
    }

    /// Get locals stack pointer (next free local slot)
    #[inline(always)]
    pub fn locals_sp(&self) -> usize {
        self.locals_sp
    }

    /// Push a new call frame
    ///
    /// Returns an error if call stack would overflow.
    #[inline(always)]
    pub fn push_frame(&mut self, return_pc: usize, locals_capacity: usize) -> Result<(), RuntimeError> {
        // Check call stack depth
        if self.depth >= self.max_depth {
            return Err(RuntimeError::CallStackOverflow {
                depth: self.depth,
            });
        }

        // Save current frame
        self.frames[self.depth] = CallFrame {
            return_pc,
            frame_base: self.frame_base,
        };
        self.depth += 1;

        // Allocate new frame
        self.frame_base = self.locals_sp;
        self.locals_sp += self.frame_size;

        // Check we haven't exceeded locals array
        if self.locals_sp > locals_capacity {
            return Err(RuntimeError::CallStackOverflow {
                depth: self.depth,
            });
        }

        Ok(())
    }

    /// Pop a call frame, returning to the previous function
    ///
    /// Returns the return PC, or None if at depth 0 (exiting main).
    #[inline(always)]
    pub fn pop_frame(&mut self) -> Option<usize> {
        if self.depth > 0 {
            // Restore frame from call stack
            self.depth -= 1;
            let frame = self.frames[self.depth];
            
            // Restore previous frame_base and deallocate current frame
            self.locals_sp = self.frame_base; // Deallocate current frame
            self.frame_base = frame.frame_base; // Restore previous frame
            
            Some(frame.return_pc)
        } else {
            // At depth 0 - exiting main
            None
        }
    }

    /// Check if we're in the main frame (depth 0)
    #[inline(always)]
    pub fn is_main_frame(&self) -> bool {
        self.depth == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_stack_creation() {
        let stack = CallStack::new(64, 32);
        assert_eq!(stack.depth(), 0);
        assert_eq!(stack.frame_base(), 0);
        assert_eq!(stack.locals_sp(), 32);
        assert!(stack.is_main_frame());
    }

    #[test]
    fn test_push_pop_frame() {
        let mut stack = CallStack::new(64, 32);

        // Push first frame
        stack.push_frame(100, 2048).unwrap();
        assert_eq!(stack.depth(), 1);
        assert_eq!(stack.frame_base(), 32);
        assert_eq!(stack.locals_sp(), 64);
        assert!(!stack.is_main_frame());

        // Push second frame
        stack.push_frame(200, 2048).unwrap();
        assert_eq!(stack.depth(), 2);
        assert_eq!(stack.frame_base(), 64);
        assert_eq!(stack.locals_sp(), 96);

        // Pop back to first frame
        let return_pc = stack.pop_frame().unwrap();
        assert_eq!(return_pc, 200);
        assert_eq!(stack.depth(), 1);
        assert_eq!(stack.frame_base(), 32);
        assert_eq!(stack.locals_sp(), 64);

        // Pop back to main
        let return_pc = stack.pop_frame().unwrap();
        assert_eq!(return_pc, 100);
        assert_eq!(stack.depth(), 0);
        assert_eq!(stack.frame_base(), 0);
        assert_eq!(stack.locals_sp(), 32);
        assert!(stack.is_main_frame());

        // Pop from main returns None
        let result = stack.pop_frame();
        assert_eq!(result, None);
    }

    #[test]
    fn test_reset() {
        let mut stack = CallStack::new(64, 32);

        // Push some frames
        stack.push_frame(100, 2048).unwrap();
        stack.push_frame(200, 2048).unwrap();

        assert_eq!(stack.depth(), 2);

        // Reset
        stack.reset();
        assert_eq!(stack.depth(), 0);
        assert_eq!(stack.frame_base(), 0);
        assert_eq!(stack.locals_sp(), 32);
        assert!(stack.is_main_frame());
    }

    #[test]
    fn test_call_stack_overflow() {
        let mut stack = CallStack::new(3, 32); // Only 3 frames max

        // Push 3 frames should work
        stack.push_frame(100, 2048).unwrap();
        stack.push_frame(200, 2048).unwrap();
        stack.push_frame(300, 2048).unwrap();

        // 4th frame should fail
        let result = stack.push_frame(400, 2048);
        assert!(matches!(
            result,
            Err(RuntimeError::CallStackOverflow { depth: 3 })
        ));
    }

    #[test]
    fn test_locals_overflow() {
        let mut stack = CallStack::new(64, 32);

        // Locals capacity is only 100, but we're trying to allocate 4 frames
        // Frame 0: 0-31
        // Frame 1: 32-63
        // Frame 2: 64-95
        // Frame 3: 96-127 (exceeds capacity of 100)
        
        stack.push_frame(100, 100).unwrap(); // 32-63: OK
        stack.push_frame(200, 100).unwrap(); // 64-95: OK
        
        let result = stack.push_frame(300, 100); // 96-127: FAIL
        assert!(matches!(
            result,
            Err(RuntimeError::CallStackOverflow { .. })
        ));
    }

    #[test]
    fn test_multiple_push_pop() {
        let mut stack = CallStack::new(64, 32);

        // Simulate multiple function calls
        for i in 0..10 {
            stack.push_frame(i * 100, 2048).unwrap();
        }
        assert_eq!(stack.depth(), 10);

        // Pop them all back
        for i in (0..10).rev() {
            let return_pc = stack.pop_frame().unwrap();
            assert_eq!(return_pc, i * 100);
        }
        assert_eq!(stack.depth(), 0);
        assert!(stack.is_main_frame());
    }
}

