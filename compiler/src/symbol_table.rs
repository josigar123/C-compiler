use std::collections::HashSet;

/*Here goes a simple symbol table, each entry will contain
  the name of the identifier, its offset on the stack that can be calculated when
  generating code or elsewhere, and the value associated with the identifer.
  The values is currently assumed to only be an integer, so that will be stored directly
*/

// Represents a single entry
// Value is optional as this: int a; is legal
// When checking when is in use, it needs to be Some(t), else fail
struct TableEntry {
    identifier: String,
    value: Option<u32>,
    stack_offset: u32,
}

// The actual structure, holds a vector of entries
// Holds a map of identifiers for checking for existence or duplicate names
// Take into account the 4 byte boundary ARM64 requires for its stack when storing ints
struct SymbolTable {
    entries: Vec<TableEntry>,
    identifier_check: HashSet<String>,
    current_stack_offet: u32,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            entries: Vec::new(),
            identifier_check: HashSet::new(),
            current_stack_offet: 0,
        }
    }

    pub fn add_entry(
        &mut self,
        identifier_name: String,
        identifier_value: Option<u32>,
    ) -> TableEntry {
        self.current_stack_offet += 4;
        let new_entry = TableEntry {
            identifier: identifier_name,
            value: identifier_value,
            stack_offset: self.current_stack_offet,
        };

        new_entry
    }
}
