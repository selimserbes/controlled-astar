#![allow(dead_code)]
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

pub struct HashTable<T> {
    capacity: usize,
    table: Vec<HashSet<T>>,
    load_factor: f64,
    size: usize,
}

impl<T> HashTable<T>
where
    T: Eq + Hash + Clone,
{
    pub fn new(capacity: usize, load_factor: f64) -> Self {
        HashTable {
            capacity,
            table: vec![HashSet::new(); capacity],
            load_factor,
            size: 0,
        }
    }

    fn hash(&self, key: &T) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % self.capacity
    }

    pub fn insert(&mut self, key: T) {
        if self.size as f64 / self.capacity as f64 > self.load_factor {
            self.resize();
        }
        let index = self.hash(&key);
        if !self.table[index].contains(&key) {
            self.table[index].insert(key);
            self.size += 1;
        }
    }

    pub fn search(&self, key: &T) -> bool {
        let index = self.hash(key);
        self.table[index].contains(key)
    }

    pub fn delete(&mut self, key: &T) {
        let index = self.hash(key);
        if self.table[index].remove(key) {
            self.size -= 1;
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn contains(&self, key: &T) -> bool {
        let index = self.hash(key);
        self.table[index].contains(key)
    }

    fn resize(&mut self) {
        let new_capacity = self.capacity * 2;
        let mut new_table = vec![HashSet::new(); new_capacity];

        for bucket in self.table.drain(..) {
            for key in bucket {
                let mut hasher = DefaultHasher::new();
                key.hash(&mut hasher);
                let index = hasher.finish() as usize % new_capacity;
                new_table[index].insert(key);
            }
        }

        self.capacity = new_capacity;
        self.table = new_table;
    }
}
