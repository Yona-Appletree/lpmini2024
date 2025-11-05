/// Compiler module
///
/// Organizes compilation into feature-grouped submodules.
pub mod expr;
pub mod func;
pub mod stmt;

pub mod ast;
pub mod codegen;
pub mod lexer;
pub mod parser;
pub mod symbol_table;
pub mod test_ast;
pub mod typechecker;
