// node.rs

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub x: usize,
    pub y: usize,
    pub is_blocked: bool,
    pub neighbors: HashMap<Direction, Option<(usize, usize)>>,
}

impl Node {
    // Node oluşturma sırasında dört yönlü komşuları otomatik atama
    pub fn new(x: usize, y: usize, is_blocked: bool, max_x: usize, max_y: usize) -> Self {
        let mut neighbors = HashMap::new();

        if y > 0 {
            neighbors.insert(Direction::North, Some((x, y - 1)));
        }
        if y < max_y {
            neighbors.insert(Direction::South, Some((x, y + 1)));
        }
        if x > 0 {
            neighbors.insert(Direction::West, Some((x - 1, y)));
        }
        if x < max_x {
            neighbors.insert(Direction::East, Some((x + 1, y)));
        }

        Node {
            x,
            y,
            is_blocked,
            neighbors,
        }
    }

    // Belirli bir yönü ayarlamak için (manuel değişiklik)
    pub fn set_neighbor(&mut self, direction: Direction, neighbor_pos: Option<(usize, usize)>) {
        self.neighbors.insert(direction, neighbor_pos);
    }

    // Belirli bir yönü kaldırmak için
    pub fn remove_neighbor(&mut self, direction: Direction) {
        self.neighbors.remove(&direction);
    }

    // Block (engel) durumunu değiştirme
    pub fn set_blocked(&mut self, blocked: bool) {
        self.is_blocked = blocked;
    }

    // Node'un yönlerini almak için
    pub fn get_directions(&self) -> Vec<Direction> {
        self.neighbors.keys().cloned().collect()
    }
}
// Hash trait'ini elle implement ediyoruz, sadece x, y ve is_blocked alanları hash'leniyor.
impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.is_blocked.hash(state);
    }
}
