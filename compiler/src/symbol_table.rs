use std::collections::HashMap;

// Represents a single entry
#[derive(Debug, PartialEq, Clone)]
pub struct TableEntry {
    pub stack_offset: u32,    // Stack offset for current variable
    pub is_initialized: bool, // Does it hold a value?
}

// Holds a map of identifiers for checking for existence or duplicate names
// Take into account the 4 byte boundary ARM64 requires for its stack when storing ints or other data
#[derive(Debug, PartialEq, Clone)]
pub struct SymbolTable {
    pub entries: HashMap<String, TableEntry>,
    pub current_stack_offet: u32, // What to be deallocated at program end
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            entries: HashMap::new(),
            current_stack_offet: 0,
        }
    }

    pub fn add_entry(&mut self, identifier: String, initialized: bool) -> TableEntry {
        // Assumes only integers, could add param to check type
        self.current_stack_offet += 4;

        let entry = TableEntry {
            stack_offset: self.current_stack_offet,
            is_initialized: initialized,
        };
        self.entries.insert(identifier, entry.clone());

        entry
    }

    pub fn pretty_print(&self) {
        println!("Symbol Table:");
        for (identifier, entry) in &self.entries {
            println!("Identifier: {}", identifier);
            println!("Stack Offset: {}", entry.stack_offset);
            println!("Initialized: {}", entry.is_initialized);
            println!();
        }
    }
}
