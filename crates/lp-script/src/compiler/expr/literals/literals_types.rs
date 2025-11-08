/// Literal expression type checking
extern crate alloc;

use crate::compiler::typechecker::TypeChecker;
use crate::shared::Type;

impl TypeChecker {
    /// Type check number literal
    ///
    /// Returns Type::Fixed.
    pub(crate) fn check_number() -> Type {
        Type::Fixed
    }

    /// Type check int literal
    ///
    /// Returns Type::Int32.
    pub(crate) fn check_int_number() -> Type {
        Type::Int32
    }
}
