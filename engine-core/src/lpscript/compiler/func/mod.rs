/// Function compilation module
/// 
/// This module contains all function-related code organized into:
/// - func_parse.rs: Function parsing logic (included in parser/mod.rs)
/// - func_gen.rs: Function code generation  
/// - func_types.rs: Function type checking
/// - func_tests.rs: Function tests (parse, gen, types)

// Note: func_parse.rs is included in parser/mod.rs to add impl methods to Parser
// It's not included here to avoid duplicate definitions
mod func_gen;
// TODO: Update func_types to use pool-based API
// mod func_types;

#[cfg(test)]
mod func_tests;

// Re-export public items
pub(crate) use func_gen::gen_function;

// Temporary FunctionTable stub until func_types is updated
pub(crate) struct FunctionTable;

pub(crate) struct FunctionSignature {
    pub(crate) params: alloc::vec::Vec<crate::lpscript::shared::Type>,
    pub(crate) return_type: crate::lpscript::shared::Type,
}

impl FunctionTable {
    pub(crate) fn new() -> Self {
        FunctionTable
    }
    
    #[allow(dead_code)]
    pub(crate) fn declare(
        &mut self,
        _name: alloc::string::String,
        _params: alloc::vec::Vec<crate::lpscript::shared::Type>,
        _return_type: crate::lpscript::shared::Type,
    ) -> Result<(), alloc::string::String> {
        // Stub - always succeed
        Ok(())
    }
    
    #[allow(dead_code)]
    pub(crate) fn lookup(&self, _name: &str) -> Option<FunctionSignature> {
        // Stub - no user-defined functions yet
        None
    }
}

