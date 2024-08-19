use std::cmp::{Ord, PartialOrd};
use std::collections::{BTreeMap, HashMap};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Direction {
    North,
    South,
    West,
    East,
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
    pub neighbors: BTreeMap<Direction, Option<(usize, usize)>>,
}

impl Node {
    // Creates a new Node and initializes its neighbors for the four cardinal directions.
    // The neighbors are assigned based on the node's position and the provided bounds (max_x, max_y).
    pub fn new(x: usize, y: usize, is_blocked: bool, max_x: usize, max_y: usize) -> Self {
        let mut neighbors = BTreeMap::new();

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

    // Sets a specific neighbor direction to the provided position.
    // Allows manual adjustments to the neighbors.
    pub fn set_neighbor(&mut self, direction: Direction, neighbor_pos: Option<(usize, usize)>) {
        self.neighbors.insert(direction, neighbor_pos);
    }

    // Removes the neighbor in the specified direction.
    pub fn remove_neighbor(&mut self, direction: Direction) {
        self.neighbors.remove(&direction);
    }

    // Sets the blocked status of the node.
    pub fn set_blocked(&mut self, blocked: bool) {
        self.is_blocked = blocked;
    }

    // Returns a vector of all directions that have neighbors.
    pub fn get_directions(&self) -> Vec<Direction> {
        self.neighbors.keys().cloned().collect()
    }
    // Converts a 2D matrix into a HashMap of Node objects.
    // 1 represents a blocked cell, 0 represents an open cell.
    pub fn matrix_to_nodes(matrix: &[Vec<i32>]) -> HashMap<(usize, usize), Node> {
        let mut hash_map = HashMap::new();
        let max_x = matrix.len() - 1;
        let max_y = matrix[0].len() - 1;

        for (x, row) in matrix.iter().enumerate() {
            for (y, &cell) in row.iter().enumerate() {
                let is_blocked = cell == 1;
                let node = Node::new(x, y, is_blocked, max_x, max_y);
                hash_map.insert((x, y), node);
            }
        }

        hash_map
    }
}

// Implements the Hash trait for Node.
// Only x, y, and is_blocked fields are used to generate the hash value.
impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.is_blocked.hash(state);
    }
}
