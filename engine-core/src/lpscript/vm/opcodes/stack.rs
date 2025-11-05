/// Stack manipulation opcodes
///
/// NOTE: These operations are now implemented directly on the Stack struct
/// in vm/vm_stack.rs and called directly from the executor.
/// This module is kept for backward compatibility and documentation.
// Re-export Stack for convenience
pub use crate::lpscript::vm::vm_stack::Stack;
