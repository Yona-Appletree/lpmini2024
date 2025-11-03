/// LPS Program definition
extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;

use super::opcodes::OpCode;
use super::locals::LocalDef;
use crate::lpscript::error::Span;

/// A compiled LightPlayer Script program
#[derive(Debug, Clone)]
pub struct LpsProgram {
    pub name: String,
    pub opcodes: Vec<OpCode>,
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

    pub fn with_opcodes(mut self, opcodes: Vec<OpCode>) -> Self {
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
}

