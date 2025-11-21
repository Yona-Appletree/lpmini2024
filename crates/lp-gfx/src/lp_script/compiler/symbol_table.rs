/// Symbol table for tracking variables in scope
extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::{format, vec};

use crate::lp_script::shared::Type;

/// Symbol table for tracking variables in scope
#[derive(Debug, Clone)]
pub(crate) struct SymbolTable {
    scopes: Vec<BTreeMap<String, Type>>,
}

impl SymbolTable {
    pub(crate) fn new() -> Self {
        SymbolTable {
            scopes: vec![BTreeMap::new()],
        }
    }

    pub(crate) fn push_scope(&mut self) {
        self.scopes.push(BTreeMap::new());
    }

    pub(crate) fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub(crate) fn declare(&mut self, name: String, ty: Type) -> Result<(), String> {
        // Check if already declared in current scope
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(&name) {
                return Err(format!(
                    "Variable '{}' already declared in this scope",
                    name
                ));
            }
            scope.insert(name, ty);
        }
        Ok(())
    }

    pub(crate) fn lookup(&self, name: &str) -> Option<Type> {
        // Search from innermost to outermost scope
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(ty.clone());
            }
        }
        None
    }

    /// Update an existing variable's type (for assignments)
    pub(crate) fn set(&mut self, name: String, ty: Type) {
        // Update in the most recent scope that has this variable
        for scope in self.scopes.iter_mut().rev() {
            use alloc::collections::btree_map::Entry;
            if let Entry::Occupied(mut entry) = scope.entry(name.clone()) {
                entry.insert(ty);
                return;
            }
        }
        // If not found, add to current scope
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, ty);
        }
    }
}
