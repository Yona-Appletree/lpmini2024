/// Parser re-export
/// 
/// This module will eventually contain the Parser struct directly,
/// but for now it re-exports from the old parser module during the transition.

// Re-export Parser from the old location
pub use crate::lpscript::parser::Parser;

