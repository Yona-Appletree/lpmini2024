/// Function table for user-defined and built-in functions
extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::lpscript::error::Type;

/// Function signature for user-defined functions
#[derive(Debug, Clone)]
pub(super) struct FunctionSignature {
    pub(super) params: Vec<Type>,
    pub(super) return_type: Type,
}

/// Function table for tracking user-defined functions
#[derive(Debug, Clone)]
pub(super) struct FunctionTable {
    functions: BTreeMap<String, FunctionSignature>,
}

impl FunctionTable {
    pub(super) fn new() -> Self {
        FunctionTable {
            functions: BTreeMap::new(),
        }
    }

    pub(super) fn declare(
        &mut self,
        name: String,
        params: Vec<Type>,
        return_type: Type,
    ) -> Result<(), String> {
        if self.functions.contains_key(&name) {
            return Err(format!("Function '{}' already declared", name));
        }
        self.functions.insert(
            name,
            FunctionSignature {
                params,
                return_type,
            },
        );
        Ok(())
    }

    pub(super) fn lookup(&self, name: &str) -> Option<&FunctionSignature> {
        self.functions.get(name)
    }
}


