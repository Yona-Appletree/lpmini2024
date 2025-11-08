/// Local variable allocation state
extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use crate::shared::Type;

pub struct LocalAllocator {
    pub(crate) locals: BTreeMap<String, u32>,
    pub(crate) local_types: BTreeMap<u32, Type>, // Track type for each local index
    pub(crate) next_index: u32,
    // Stack of scopes, each scope contains variables declared in that scope
    // and their previous index (if they shadowed an outer variable)
    scope_stack: Vec<Vec<(String, Option<u32>)>>,
}

impl LocalAllocator {
    pub fn new() -> Self {
        LocalAllocator {
            locals: BTreeMap::new(),
            local_types: BTreeMap::new(),
            next_index: 0,
            scope_stack: Vec::new(),
        }
    }

    pub fn allocate(&mut self, name: String) -> u32 {
        let previous_index = self.locals.get(&name).copied();

        // Always allocate a new index for this variable
        let index = self.next_index;
        self.next_index += 1;
        self.locals.insert(name.clone(), index);

        // If we're in a scope, track this variable
        if let Some(current_scope) = self.scope_stack.last_mut() {
            current_scope.push((name, previous_index));
        }

        index
    }

    pub fn allocate_typed(&mut self, name: String, ty: Type) -> u32 {
        let index = self.allocate(name);
        self.local_types.insert(index, ty);
        index
    }

    pub fn get(&self, name: &str) -> Option<u32> {
        self.locals.get(name).copied()
    }

    pub fn get_type(&self, index: u32) -> Option<&Type> {
        self.local_types.get(&index)
    }

    /// Push a new scope (e.g., entering a block)
    pub fn push_scope(&mut self) {
        self.scope_stack.push(Vec::new());
    }

    /// Pop a scope (e.g., exiting a block), restoring shadowed variables
    pub fn pop_scope(&mut self) {
        if let Some(scope) = self.scope_stack.pop() {
            // Restore previous variable bindings
            for (name, previous_index) in scope {
                if let Some(prev_idx) = previous_index {
                    // Restore the shadowed variable
                    self.locals.insert(name, prev_idx);
                } else {
                    // Remove the variable (it was declared in this scope)
                    self.locals.remove(&name);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_allocation() {
        let mut allocator = LocalAllocator::new();

        let x_idx = allocator.allocate(String::from("x"));
        let y_idx = allocator.allocate(String::from("y"));
        let z_idx = allocator.allocate(String::from("z"));

        assert_eq!(x_idx, 0);
        assert_eq!(y_idx, 1);
        assert_eq!(z_idx, 2);
    }

    #[test]
    fn test_variable_lookup() {
        let mut allocator = LocalAllocator::new();

        let x_idx = allocator.allocate(String::from("x"));
        let y_idx = allocator.allocate(String::from("y"));

        assert_eq!(allocator.get("x"), Some(x_idx));
        assert_eq!(allocator.get("y"), Some(y_idx));
        assert_eq!(allocator.get("z"), None);
    }

    #[test]
    fn test_scope_push_pop() {
        let mut allocator = LocalAllocator::new();

        // Outer scope
        let x_idx = allocator.allocate(String::from("x"));
        assert_eq!(x_idx, 0);
        assert_eq!(allocator.get("x"), Some(0));

        // Enter inner scope
        allocator.push_scope();
        let y_idx = allocator.allocate(String::from("y"));
        assert_eq!(y_idx, 1);
        assert_eq!(allocator.get("x"), Some(0)); // outer var still visible
        assert_eq!(allocator.get("y"), Some(1));

        // Exit inner scope
        allocator.pop_scope();
        assert_eq!(allocator.get("x"), Some(0)); // outer var still there
        assert_eq!(allocator.get("y"), None); // inner var removed
    }

    #[test]
    fn test_variable_shadowing() {
        let mut allocator = LocalAllocator::new();

        // Outer scope: declare x
        let outer_x = allocator.allocate(String::from("x"));
        assert_eq!(outer_x, 0);
        assert_eq!(allocator.get("x"), Some(0));

        // Enter inner scope and shadow x
        allocator.push_scope();
        let inner_x = allocator.allocate(String::from("x"));
        assert_eq!(inner_x, 1); // new allocation
        assert_eq!(allocator.get("x"), Some(1)); // inner x shadows outer

        // Exit inner scope - outer x should be restored
        allocator.pop_scope();
        assert_eq!(allocator.get("x"), Some(0)); // back to outer x
    }

    #[test]
    fn test_nested_scopes() {
        let mut allocator = LocalAllocator::new();

        // Level 0: x = 0
        let x0 = allocator.allocate(String::from("x"));
        assert_eq!(x0, 0);

        // Level 1: shadow x
        allocator.push_scope();
        let x1 = allocator.allocate(String::from("x"));
        assert_eq!(x1, 1);
        assert_eq!(allocator.get("x"), Some(1));

        // Level 2: shadow x again
        allocator.push_scope();
        let x2 = allocator.allocate(String::from("x"));
        assert_eq!(x2, 2);
        assert_eq!(allocator.get("x"), Some(2));

        // Pop level 2
        allocator.pop_scope();
        assert_eq!(allocator.get("x"), Some(1)); // back to level 1

        // Pop level 1
        allocator.pop_scope();
        assert_eq!(allocator.get("x"), Some(0)); // back to level 0
    }

    #[test]
    fn test_multiple_variables_in_scope() {
        let mut allocator = LocalAllocator::new();

        // Outer scope
        allocator.allocate(String::from("a"));
        allocator.allocate(String::from("b"));

        // Inner scope with multiple variables
        allocator.push_scope();
        allocator.allocate(String::from("c"));
        allocator.allocate(String::from("d"));
        allocator.allocate(String::from("e"));

        assert_eq!(allocator.get("a"), Some(0));
        assert_eq!(allocator.get("b"), Some(1));
        assert_eq!(allocator.get("c"), Some(2));
        assert_eq!(allocator.get("d"), Some(3));
        assert_eq!(allocator.get("e"), Some(4));

        // Pop scope - c, d, e should be removed
        allocator.pop_scope();
        assert_eq!(allocator.get("a"), Some(0));
        assert_eq!(allocator.get("b"), Some(1));
        assert_eq!(allocator.get("c"), None);
        assert_eq!(allocator.get("d"), None);
        assert_eq!(allocator.get("e"), None);
    }

    #[test]
    fn test_shadowing_restoration() {
        let mut allocator = LocalAllocator::new();

        // Outer: x=0, y=1
        allocator.allocate(String::from("x"));
        allocator.allocate(String::from("y"));

        // Inner: shadow x, add z
        allocator.push_scope();
        allocator.allocate(String::from("x")); // shadow x (now index 2)
        allocator.allocate(String::from("z")); // new var (index 3)

        assert_eq!(allocator.get("x"), Some(2)); // shadowed
        assert_eq!(allocator.get("y"), Some(1)); // unchanged
        assert_eq!(allocator.get("z"), Some(3)); // new

        // Pop: x should restore to 0, z should disappear, y unchanged
        allocator.pop_scope();
        assert_eq!(allocator.get("x"), Some(0)); // restored
        assert_eq!(allocator.get("y"), Some(1)); // unchanged
        assert_eq!(allocator.get("z"), None); // removed
    }

    #[test]
    fn test_empty_scope() {
        let mut allocator = LocalAllocator::new();

        allocator.allocate(String::from("x"));

        // Push and immediately pop an empty scope
        allocator.push_scope();
        allocator.pop_scope();

        // x should still be there
        assert_eq!(allocator.get("x"), Some(0));
    }
}
