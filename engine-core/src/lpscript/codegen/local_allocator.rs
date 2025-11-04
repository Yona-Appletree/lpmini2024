/// Local variable allocation state
extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;

pub struct LocalAllocator {
    pub(crate) locals: BTreeMap<String, u32>,
    pub(crate) next_index: u32,
}

impl LocalAllocator {
    pub fn new() -> Self {
        LocalAllocator {
            locals: BTreeMap::new(),
            next_index: 0,
        }
    }
    
    pub fn allocate(&mut self, name: String) -> u32 {
        if let Some(&index) = self.locals.get(&name) {
            index
        } else {
            let index = self.next_index;
            self.next_index += 1;
            self.locals.insert(name, index);
            index
        }
    }
    
    pub fn get(&self, name: &str) -> Option<u32> {
        self.locals.get(name).copied()
    }
}

