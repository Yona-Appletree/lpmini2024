/// LPS Program definition
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

use super::locals::LocalDef;
use super::opcodes::LpsOpCode;
use crate::lpscript::shared::Span;

/// A compiled LightPlayer Script program
#[derive(Debug, Clone)]
pub struct LpsProgram {
    pub name: String,
    pub opcodes: Vec<LpsOpCode>,
    pub locals: Vec<LocalDef>,
    pub source_map: Option<Vec<Span>>,
    pub source: Option<String>,
}

impl LpsProgram {
    pub fn new(name: String) -> Self {
        LpsProgram {
            name,
            opcodes: Vec::new(),
            locals: Vec::new(),
            source_map: None,
            source: None,
        }
    }

    pub fn with_opcodes(mut self, opcodes: Vec<LpsOpCode>) -> Self {
        self.opcodes = opcodes;
        self
    }

    pub fn with_source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }

    pub fn with_source_map(mut self, source_map: Vec<Span>) -> Self {
        self.source_map = Some(source_map);
        self
    }

    pub fn with_locals(mut self, locals: Vec<LocalDef>) -> Self {
        self.locals = locals;
        self
    }
}
