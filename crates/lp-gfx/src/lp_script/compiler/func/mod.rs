/// Function compilation module
///
/// This module contains all function-related code organized into:
/// - func_parse.rs: Function parsing logic (included in parser/mod.rs)
/// - func_gen.rs: Function code generation  
/// - func_types.rs: Function type checking
/// - func_tests.rs: Function tests (parse, gen, types)
// Note: func_parse.rs is included in parser/mod.rs to add impl methods to Parser
// It's not included here to avoid duplicate definitions
pub(crate) mod func_gen;
mod func_types;

#[cfg(test)]
mod func_tests;

// Re-export public items
pub(crate) use func_types::{FunctionMetadata, FunctionTable, LocalVarInfo};
