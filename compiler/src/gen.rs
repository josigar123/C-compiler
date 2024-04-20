/*
    Will generate the assembly based on the root of the Parse tree
*/
use crate::magic_parsing::{self, ProgramNode};

// A generator struct that has a program node for its root.
pub struct Generator {
    root: ProgramNode,
}

impl Generator {
    pub fn new(root_node: ProgramNode) -> Self {
        Generator { root: root_node }
    }
}
