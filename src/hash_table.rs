use crate::node::Node;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

pub struct HashTable {
    table: Vec<HashSet<Node>>,
    capacity: usize,
}

impl HashTable {
    pub fn new(capacity: usize) -> Self {
        HashTable {
            table: vec![HashSet::new(); capacity],
            capacity,
        }
    }

    fn hash(&self, key: &Node) -> usize {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % self.capacity
    }

    pub fn insert(&mut self, key: Node) {
        let index = self.hash(&key);
        self.table[index].insert(key);
    }

    pub fn search(&self, key: &Node) -> bool {
        let index = self.hash(key);
        self.table[index].contains(key)
    }

    pub fn delete(&mut self, key: &Node) {
        let index = self.hash(key);
        self.table[index].remove(key);
    }

    pub fn len(&self) -> usize {
        self.table.iter().map(|bucket| bucket.len()).sum()
    }
}
