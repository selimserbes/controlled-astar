use crate::node::Node;
use std::collections::HashSet;

pub struct HashTable {
    set: HashSet<Node>,
}

impl HashTable {
    pub fn new() -> Self {
        HashTable {
            set: HashSet::new(),
        }
    }

    pub fn insert(&mut self, node: Node) {
        self.set.insert(node);
    }

    pub fn search(&self, node: &Node) -> bool {
        self.set.contains(node)
    }
}
