use std::cmp::{Ord, PartialOrd};
use std::collections::{BTreeMap, HashMap};
use std::hash::{Hash, Hasher};

/// Directions used in nodes.
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

/// Represents a node on a map.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub x: usize,
    pub y: usize,
    pub is_blocked: bool,
    pub neighbors: BTreeMap<Direction, Option<(usize, usize)>>,
}

impl Node {
    /// Creates a new `Node` and initializes neighbors for the four basic directions.
    ///
    /// # Parameters
    /// - `x`: The x-coordinate of the node.
    /// - `y`: The y-coordinate of the node.
    /// - `is_blocked`: Indicates whether the node is blocked.
    /// - `max_x`: The maximum x dimension of the map.
    /// - `max_y`: The maximum y dimension of the map.
    ///
    /// # Returns
    /// A newly created `Node` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_crate::Node;
    ///
    /// // Create a new `Node` at position (2, 3) which is not blocked
    /// let node = Node::new(2, 3, false, 10, 10);
    ///
    /// // Check the node's coordinates and blocked status
    /// assert_eq!(node.x, 2);
    /// assert_eq!(node.y, 3);
    /// assert_eq!(node.is_blocked, false);
    /// ```
    pub fn new(x: usize, y: usize, is_blocked: bool, max_x: usize, max_y: usize) -> Self {
        let mut neighbors = BTreeMap::new();

        // Initialize neighbors for the four basic directions
        if y > 0 {
            // North neighbor exists if y > 0
            neighbors.insert(Direction::North, Some((x, y - 1)));
        }
        if y < max_y {
            // South neighbor exists if y < max_y
            neighbors.insert(Direction::South, Some((x, y + 1)));
        }
        if x > 0 {
            // West neighbor exists if x > 0
            neighbors.insert(Direction::West, Some((x - 1, y)));
        }
        if x < max_x {
            // East neighbor exists if x < max_x
            neighbors.insert(Direction::East, Some((x + 1, y)));
        }

        Node {
            x,
            y,
            is_blocked,
            neighbors,
        }
    }

    /// Sets the neighbor position for a specific direction.
    ///
    /// # Parameters
    /// - `direction`: The direction for which to set the neighbor.
    /// - `neighbor_pos`: The position of the neighbor.
    ///
    /// # Example
    /// ```rust
    /// let mut node = Node::new(0, 0, false, 10, 10);
    /// node.set_neighbor(Direction::North, Some((0, 1)));
    /// ```
    pub fn set_neighbor(&mut self, direction: Direction, neighbor_pos: Option<(usize, usize)>) {
        // Update the neighbor position for the given direction
        self.neighbors.insert(direction, neighbor_pos);
    }

    /// Removes the neighbor for a specific direction.
    ///
    /// # Parameters
    /// - `direction`: The direction for which to remove the neighbor.
    ///
    /// # Example
    /// ```rust
    /// let mut node = Node::new(0, 0, false, 10, 10);
    /// node.remove_neighbor(Direction::North);
    /// ```
    pub fn remove_neighbor(&mut self, direction: Direction) {
        // Remove the neighbor in the specified direction
        self.neighbors.remove(&direction);
    }

    /// Sets whether the node is blocked or not.
    ///
    /// # Parameters
    /// - `blocked`: The blocked status.
    ///
    /// # Example
    /// ```rust
    /// let mut node = Node::new(0, 0, false, 10, 10);
    /// node.set_blocked(true);
    /// ```
    pub fn set_blocked(&mut self, blocked: bool) {
        // Update the blocked status of the node
        self.is_blocked = blocked;
    }

    /// Returns a vector of directions where neighbors are present.
    ///
    /// # Returns
    /// A vector containing the directions of neighbors.
    ///
    /// # Example
    /// ```rust
    /// let node = Node::new(0, 0, false, 10, 10);
    /// let directions = node.get_directions();
    /// ```
    pub fn get_directions(&self) -> Vec<Direction> {
        // Collect and return the directions of existing neighbors
        self.neighbors.keys().cloned().collect()
    }

    /// Converts a 2D grid into `Node` objects.
    ///
    /// # Parameters
    /// - `grid`: The 2D grid where `1` represents a blocked node and `0` represents a free node.
    ///
    /// # Returns
    /// A `HashMap` containing `Node` objects mapped by their positions.
    ///
    /// # Example
    /// ```rust
    /// let grid = vec![
    ///     vec![0, 1, 0],
    ///     vec![0, 0, 1],
    /// ];
    /// let nodes = Node::grid_to_nodes(&grid);
    /// ```
    pub fn grid_to_nodes(grid: &[Vec<i32>]) -> HashMap<(usize, usize), Node> {
        let mut hash_map = HashMap::new();
        let max_x = grid.len() - 1;
        let max_y = grid[0].len() - 1;

        // Iterate over the grid to create nodes
        for (x, row) in grid.iter().enumerate() {
            for (y, &cell) in row.iter().enumerate() {
                let is_blocked = cell == 1;
                // Create a new Node and insert it into the HashMap
                let node = Node::new(x, y, is_blocked, max_x, max_y);
                hash_map.insert((x, y), node);
            }
        }

        hash_map
    }

    /// Prints a 2D grid and an optional path to the screen.
    ///
    /// # Parameters
    /// - `grid`: The 2D grid.
    /// - `path`: An optional vector representing the path to be printed on the grid.
    ///
    /// # Example
    /// ```rust
    /// let grid = vec![
    ///     vec![0, 0, 0],
    ///     vec![0, 1, 0],
    /// ];
    /// let path = Some(vec![(0, 0), (1, 1)]);
    /// Node::print_grid(&grid, &path);
    /// ```
    pub fn print_grid(grid: &[Vec<i32>], path: &Option<Vec<(usize, usize)>>) {
        // Iterate over each row in the grid
        for (y, row) in grid.iter().enumerate() {
            // Iterate over each cell in the row
            for (x, &cell) in row.iter().enumerate() {
                // Check if the path is defined
                if let Some(ref p) = *path {
                    // Print 'o' if the cell is part of the path
                    if p.contains(&(x, y)) {
                        print!("o ");
                    } else if cell == 1 {
                        // Print '#' if the cell is an obstacle
                        print!("# ");
                    } else {
                        // Print '.' if the cell is free
                        print!(". ");
                    }
                } else {
                    // If no path is provided, just print '.'
                    print!(". ");
                }
            }
            // Print a newline after each row
            println!();
        }
    }
}

// Implements the `Hash` trait for `Node`.
// Only x, y, and is_blocked fields are used to create the hash value.
impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash the x-coordinate
        self.x.hash(state);
        // Hash the y-coordinate
        self.y.hash(state);
        // Hash the blocked status
        self.is_blocked.hash(state);
    }
}
