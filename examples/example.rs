//! # A* Pathfinding Example
//!
//! This example demonstrates finding the shortest path using the A* algorithm
//! on a 10x10 grid. The grid consists of open (0) and blocked (1) cells.
//! Users can manually adjust neighbors of specific nodes and find the shortest
//! path from a start point to a goal point.

use controlled_astar::{AStar, Direction, Node};

/// # Main Function
///
/// This function defines a 10x10 grid, converts it into `Node` objects,
/// manually adjusts the neighbors of specific nodes, and then uses the A*
/// algorithm to find the shortest path.
fn main() {
    // Define a 10x10 grid.
    // This matrix uses 0 for open cells and 1 for blocked cells.
    let matrix = vec![
        vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 1, 0, 1, 1, 1, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
        vec![0, 0, 0, 1, 1, 1, 0, 0, 0, 0],
        vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 1, 0, 0, 0, 1, 0],
        vec![0, 1, 0, 0, 0, 0, 0, 0, 1, 0],
        vec![0, 1, 1, 1, 0, 1, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 1, 1, 0, 0, 1, 1, 0],
    ];

    // Convert the matrix to a HashMap of `Node` objects.
    // Each cell in the matrix is represented by a `Node` object.
    let mut nodes = Node::matrix_to_nodes(&matrix);

    // Uncomment this block if you want to manually modify specific nodes.

    let position = (0, 0);

    if let Some(node) = nodes.get_mut(&position) {
        // Removes the southern and eastern neighbors.
        node.remove_neighbor(Direction::South);
        node.remove_neighbor(Direction::East);
        // Adds a southeastern neighbor.
        node.set_neighbor(Direction::SouthEast, Some((position.0 + 1, position.1 + 1)));
    }

    let position_2 = (1, 1);

    if let Some(node) = nodes.get_mut(&position_2) {
        // Sets the southeastern neighbor for this node.
        node.set_neighbor(
            Direction::SouthEast,
            Some((position_2.0 + 1, position_2.1 + 1)),
        );
    }

    // Print the directions of all nodes.
    // This will show which neighbors each node has.
    for ((x, y), node) in nodes.iter() {
        println!("Node ({}, {}): {:?}", x, y, node.get_directions());
    }

    // Print the directions of the specific node at position (0, 0).
    // Here, it shows the neighbors of the node at position (1, 1).
    if let Some(node) = nodes.get(&(1, 1)) {
        println!("Node ({}, {}): {:?}", 1, 1, node.get_directions());
    }

    // Initialize AStar with the nodes map.
    let mut astar = AStar::new(nodes);

    // Example: Find the shortest path from (0, 0) to (9, 9).
    // Defines the start and goal positions and uses `find_shortest_path`.
    let start = (0, 0);
    let goal = (9, 9);
    let path = astar.find_shortest_path(start, goal);
    Node::print_matrix(&matrix, &path);

    // Print the path.
    match path {
        Some(path) => {
            println!("Path: {:?}", path);
        }
        None => {
            println!("No path found from {:?} to {:?}", start, goal);
        }
    }
}
