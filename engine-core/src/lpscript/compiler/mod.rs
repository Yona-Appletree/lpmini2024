/// Compiler module - groups all compiler phases by feature
///
/// This is a new organization that groups parsing, code generation, and type checking
/// by language feature rather than by compiler phase.

pub mod parser;
pub mod generator;

pub mod expr;

