use alloc::vec::Vec;

/// Call stack management for LPS VM function calls
use super::error::LpsVmError;

/// Call frame for function calls
///
/// Tracks where to return and how to restore the locals allocation state.
#[derive(Debug, Clone, Copy, Default)]
pub struct CallFrame {
    pub return_pc: usize,         // PC to return to
    pub return_fn_idx: usize,     // Function index to return to
    pub frame_base: usize,        // Base local index for this frame
    pub locals_restore_sp: usize, // Local count to restore on return
}

/// Call stack for managing function call frames
///
/// Pre-allocates frames to avoid runtime allocations during execution.
/// Supports variable-sized frames - each function can allocate the exact
/// number of locals it needs, not a fixed size.
pub struct CallStack {
    frames: Vec<CallFrame>,
    depth: usize,
    max_depth: usize,
    frame_base: usize,     // Current frame's base local index
    current_fn_idx: usize, // Current function index
}

impl CallStack {
    /// Create a new call stack
    ///
    /// # Arguments
    /// * `max_depth` - Maximum call depth (e.g., 64)
    pub fn try_new(max_depth: usize) -> Result<Self, LpsVmError> {
        let mut frames = Vec::new();
        if max_depth > 0 {
            frames.reserve(max_depth);
            for _ in 0..max_depth {
                frames.push(CallFrame::default());
            }
        }

        Ok(CallStack {
            frames,
            depth: 0,
            max_depth,
            frame_base: 0,
            current_fn_idx: 0,
        })
    }

    pub fn new(max_depth: usize) -> Self {
        Self::try_new(max_depth).expect("call stack allocation failed")
    }

    /// Reset the call stack for a new execution
    #[inline(always)]
    pub fn reset(&mut self, _main_locals_count: usize) {
        self.depth = 0;
        self.frame_base = 0;
        self.current_fn_idx = 0;
        // Frame base starts after main's locals
        // (main is always at depth 0, so no frame to save)
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

    /// Get current function index
    #[inline(always)]
    pub fn current_fn_idx(&self) -> usize {
        self.current_fn_idx
    }

    /// Push a new call frame
    ///
    /// # Arguments
    /// * `return_pc` - PC to return to after function completes
    /// * `return_fn_idx` - Function index to return to
    /// * `new_frame_base` - Base local index for the new frame
    /// * `current_locals_sp` - Current local count (to restore on return)
    /// * `new_fn_idx` - Function index being called
    ///
    /// Returns an error if call stack would overflow.
    #[inline(always)]
    pub fn push_frame(
        &mut self,
        return_pc: usize,
        return_fn_idx: usize,
        new_frame_base: usize,
        current_locals_sp: usize,
        new_fn_idx: usize,
    ) -> Result<(), LpsVmError> {
        // Check call stack depth
        if self.depth >= self.max_depth {
            return Err(LpsVmError::CallStackOverflow { depth: self.depth });
        }

        // Save current frame state
        self.frames[self.depth] = CallFrame {
            return_pc,
            return_fn_idx,
            frame_base: self.frame_base,
            locals_restore_sp: current_locals_sp,
        };
        self.depth += 1;

        // Set up new frame
        self.frame_base = new_frame_base;
        self.current_fn_idx = new_fn_idx;

        Ok(())
    }

    /// Pop a call frame, returning to the previous function
    ///
    /// Returns (return_pc, return_fn_idx, locals_restore_sp), or None if at depth 0 (exiting main).
    #[inline(always)]
    pub fn pop_frame(&mut self) -> Option<(usize, usize, usize)> {
        if self.depth > 0 {
            // Restore frame from call stack
            self.depth -= 1;
            let frame = self.frames[self.depth];

            // Restore previous frame state
            self.frame_base = frame.frame_base;
            self.current_fn_idx = frame.return_fn_idx;

            Some((
                frame.return_pc,
                frame.return_fn_idx,
                frame.locals_restore_sp,
            ))
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
        let stack = CallStack::new(64);
        assert_eq!(stack.depth(), 0);
        assert_eq!(stack.frame_base(), 0);
        assert_eq!(stack.current_fn_idx(), 0);
        assert!(stack.is_main_frame());
    }

    #[test]
    fn test_push_pop_frame() {
        let mut stack = CallStack::new(64);

        // Main function has 3 locals (indices 0, 1, 2)
        // Push first frame - function 1 with locals starting at index 3
        stack.push_frame(100, 0, 3, 3, 1).unwrap();
        assert_eq!(stack.depth(), 1);
        assert_eq!(stack.frame_base(), 3);
        assert_eq!(stack.current_fn_idx(), 1);
        assert!(!stack.is_main_frame());

        // Function 1 has 5 locals (indices 3-7)
        // Push second frame - function 2 with locals starting at index 8
        stack.push_frame(200, 1, 8, 8, 2).unwrap();
        assert_eq!(stack.depth(), 2);
        assert_eq!(stack.frame_base(), 8);
        assert_eq!(stack.current_fn_idx(), 2);

        // Pop back to function 1
        let result = stack.pop_frame().unwrap();
        assert_eq!(result, (200, 1, 8)); // return_pc, return_fn_idx, locals_restore_sp
        assert_eq!(stack.depth(), 1);
        assert_eq!(stack.frame_base(), 3);
        assert_eq!(stack.current_fn_idx(), 1);

        // Pop back to main
        let result = stack.pop_frame().unwrap();
        assert_eq!(result, (100, 0, 3));
        assert_eq!(stack.depth(), 0);
        assert_eq!(stack.frame_base(), 0);
        assert_eq!(stack.current_fn_idx(), 0);
        assert!(stack.is_main_frame());

        // Pop from main returns None
        let result = stack.pop_frame();
        assert_eq!(result, None);
    }

    #[test]
    fn test_reset() {
        let mut stack = CallStack::new(64);

        // Push some frames
        stack.push_frame(100, 0, 3, 3, 1).unwrap();
        stack.push_frame(200, 1, 8, 8, 2).unwrap();

        assert_eq!(stack.depth(), 2);

        // Reset
        stack.reset(3);
        assert_eq!(stack.depth(), 0);
        assert_eq!(stack.frame_base(), 0);
        assert_eq!(stack.current_fn_idx(), 0);
        assert!(stack.is_main_frame());
    }

    #[test]
    fn test_call_stack_overflow() {
        let mut stack = CallStack::new(3); // Only 3 frames max

        // Push 3 frames should work
        stack.push_frame(100, 0, 3, 3, 1).unwrap();
        stack.push_frame(200, 1, 8, 8, 2).unwrap();
        stack.push_frame(300, 2, 13, 13, 3).unwrap();

        // 4th frame should fail
        let result = stack.push_frame(400, 3, 18, 18, 4);
        assert!(matches!(
            result,
            Err(LpsVmError::CallStackOverflow { depth: 3 })
        ));
    }

    #[test]
    fn test_variable_sized_frames() {
        let mut stack = CallStack::new(64);

        // Main: 2 locals (0-1), sp=2
        // Call func1: 3 locals (2-4), sp=5
        stack.push_frame(100, 0, 2, 2, 1).unwrap();
        assert_eq!(stack.frame_base(), 2);

        // Call func2: 7 locals (5-11), sp=12
        stack.push_frame(200, 1, 5, 5, 2).unwrap();
        assert_eq!(stack.frame_base(), 5);

        // Call func3: 1 local (12), sp=13
        stack.push_frame(300, 2, 12, 12, 3).unwrap();
        assert_eq!(stack.frame_base(), 12);

        // Pop func3, restore to sp=12
        let (_, _, restore_sp) = stack.pop_frame().unwrap();
        assert_eq!(restore_sp, 12);
        assert_eq!(stack.frame_base(), 5);

        // Pop func2, restore to sp=5
        let (_, _, restore_sp) = stack.pop_frame().unwrap();
        assert_eq!(restore_sp, 5);
        assert_eq!(stack.frame_base(), 2);

        // Pop func1, restore to sp=2
        let (_, _, restore_sp) = stack.pop_frame().unwrap();
        assert_eq!(restore_sp, 2);
        assert_eq!(stack.frame_base(), 0);
    }

    #[test]
    fn test_multiple_push_pop() {
        let mut stack = CallStack::new(64);

        // Simulate multiple function calls with varying local counts
        let mut current_sp = 0;
        let mut frame_bases = vec![];

        for i in 0..10 {
            let locals_count = (i + 1) * 2; // Varying sizes: 2, 4, 6, 8, ...
            frame_bases.push(current_sp);
            stack
                .push_frame(i * 100, i, current_sp, current_sp, i + 1)
                .unwrap();
            current_sp += locals_count;
        }
        assert_eq!(stack.depth(), 10);

        // Pop them all back
        for i in (0..10).rev() {
            let (return_pc, _, _) = stack.pop_frame().unwrap();
            assert_eq!(return_pc, i * 100);
        }
        assert_eq!(stack.depth(), 0);
        assert!(stack.is_main_frame());
    }
}
