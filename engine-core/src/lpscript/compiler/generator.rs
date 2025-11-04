/// Code generator re-export
/// 
/// This module will eventually contain the CodeGenerator struct directly,
/// but for now it re-exports from the old codegen module during the transition.

// Re-export CodeGenerator from the old location
pub use crate::lpscript::codegen::CodeGenerator;

