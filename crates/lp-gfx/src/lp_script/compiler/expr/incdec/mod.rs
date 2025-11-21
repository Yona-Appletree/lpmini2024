/// Increment/Decrement operators (++, --)
///
/// These operators work on Dec32 and Int32 types and can be used as prefix or postfix.
mod incdec_parse;
// mod incdec_types; // Old Box-based type checker (tests disabled - use incdec_tests.rs)
mod incdec_gen;

#[cfg(test)]
mod incdec_tests;
