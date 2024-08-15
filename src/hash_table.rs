use std::collections::HashMap;
use std::hash::Hash;

const INITIAL_CAPACITY: usize = 16;
const LOAD_FACTOR: f32 = 0.75;

pub struct HashTable<K, V> {
    table: HashMap<K, V>,
    load_factor: f32,
}

impl<K, V> HashTable<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    pub fn new() -> Self {
        HashTable {
            table: HashMap::with_capacity(INITIAL_CAPACITY),
            load_factor: LOAD_FACTOR,
        }
    }

    // Yük faktörünü kontrol ederek yeniden boyutlandırmayı başlatır.
    fn check_and_resize(&mut self) {
        let capacity = self.table.capacity();
        let size = self.table.len();

        if size as f32 / capacity as f32 > self.load_factor {
            self.rehash();
        }
    }

    // Kapasiteyi iki katına çıkarır ve mevcut elemanları yeniden ekler.
    fn rehash(&mut self) {
        let new_capacity = self.table.capacity() * 2;
        let mut new_table = HashMap::with_capacity(new_capacity);

        // Eski tabloyu yeni tablo ile değiştir
        std::mem::swap(&mut new_table, &mut self.table);

        // Yeni tabloyu güncelle
        for (key, value) in new_table.drain() {
            self.table.insert(key, value);
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.check_and_resize();
        self.table.insert(key, value);
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.table.get(key)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.table.get_mut(key)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.table.remove(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.table.iter()
    }
}
