/// LPS Program definition
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

use super::opcodes::LpsOpCode;
use crate::shared::{Span, Type};

/// A compiled LightPlayer Script program
/// 
/// New structure: All code is organized into functions.
/// functions[0] is always "main" (the entry point).
#[derive(Debug, Clone)]
pub struct LpsProgram {
    pub name: String,
    pub functions: Vec<FunctionDef>,
    pub source_map: Option<Vec<Span>>,
    pub source: Option<String>,
    
    // Legacy fields (deprecated, kept for backward compatibility during migration)
    #[deprecated(note = "Use functions instead")]
    pub opcodes: Vec<LpsOpCode>,
    #[deprecated(note = "Use functions[0].locals instead")]
    pub locals: Vec<LocalDef>,
}

impl LpsProgram {
    /// Create a new empty program
    pub fn new(name: String) -> Self {
        #[allow(deprecated)]
        LpsProgram {
            name,
            functions: Vec::new(),
            source_map: None,
            source: None,
            opcodes: Vec::new(),
            locals: Vec::new(),
        }
    }

    /// Create a program with functions (new API)
    pub fn with_functions(mut self, functions: Vec<FunctionDef>) -> Self {
        self.functions = functions;
        self
    }

    /// Get the main function (always at index 0)
    pub fn main_function(&self) -> Option<&FunctionDef> {
        self.functions.get(0)
    }

    /// Get a function by index
    pub fn function(&self, idx: usize) -> Option<&FunctionDef> {
        self.functions.get(idx)
    }

    pub fn with_source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }

    pub fn with_source_map(mut self, source_map: Vec<Span>) -> Self {
        self.source_map = Some(source_map);
        self
    }

    // Legacy API (deprecated)
    #[deprecated(note = "Use with_functions instead")]
    pub fn with_opcodes(mut self, opcodes: Vec<LpsOpCode>) -> Self {
        #[allow(deprecated)]
        {
            self.opcodes = opcodes;
        }
        self
    }

    #[deprecated(note = "Use with_functions instead")]
    pub fn with_locals(mut self, locals: Vec<LocalDef>) -> Self {
        #[allow(deprecated)]
        {
            self.locals = locals;
        }
        self
    }
}

/// Function parameter definition
#[derive(Debug, Clone)]
pub struct ParamDef {
    pub name: String,
    pub ty: Type,
}

impl ParamDef {
    pub fn new(name: String, ty: Type) -> Self {
        ParamDef { name, ty }
    }
}

/// Local variable definition (metadata only, for compiled functions)
#[derive(Debug, Clone)]
pub struct LocalVarDef {
    pub name: String,
    pub ty: Type,
    pub initial_value: Option<Vec<i32>>, // Optional initial value (raw i32 representation)
}

impl LocalVarDef {
    pub fn new(name: String, ty: Type) -> Self {
        LocalVarDef {
            name,
            ty,
            initial_value: None,
        }
    }

    pub fn with_initial_value(mut self, value: Vec<i32>) -> Self {
        self.initial_value = Some(value);
        self
    }
}

// Type alias for backward compatibility during migration
pub type LocalDef = LocalVarDef;

/// Compiled function definition
#[derive(Debug, Clone)]
pub struct FunctionDef {
    pub name: String,
    pub return_type: Type,
    pub params: Vec<ParamDef>,
    pub locals: Vec<LocalDef>,
    pub opcodes: Vec<LpsOpCode>,
}

impl FunctionDef {
    pub fn new(name: String, return_type: Type) -> Self {
        FunctionDef {
            name,
            return_type,
            params: Vec::new(),
            locals: Vec::new(),
            opcodes: Vec::new(),
        }
    }

    pub fn with_params(mut self, params: Vec<ParamDef>) -> Self {
        self.params = params;
        self
    }

    pub fn with_locals(mut self, locals: Vec<LocalDef>) -> Self {
        self.locals = locals;
        self
    }

    pub fn with_opcodes(mut self, opcodes: Vec<LpsOpCode>) -> Self {
        self.opcodes = opcodes;
        self
    }
}
