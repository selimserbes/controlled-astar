//! # A* Pathfinding Example
//!
//! This example demonstrates how to use the A* pathfinding algorithm to find
//! the shortest path on a 10x10 grid. The grid consists of open cells (0)
//! and blocked cells (1). Users can manually adjust the neighbors and block
//! statuses of specific nodes and find the shortest path from a start point to
//! a goal point.
//!
//! The example includes:
//! - Defining a 10x10 grid with open and blocked cells
//! - Converting the grid into a map of `Node` objects
//! - Manually adjusting neighbors and blocking nodes
//! - Using the A* algorithm to find the shortest path from a start to a goal
//! - Printing the grid with the found path and handling any errors

use controlled_astar::{AStar, AStarError, Direction, Node};

/// # Main Function
///
/// This function sets up a 10x10 grid, converts it into `Node` objects, and
/// then demonstrates finding the shortest path using the A* algorithm.
///
/// # grid Definition
/// The grid is defined as a 2D vector where:
/// - `0` represents an open cell
/// - `1` represents a blocked cell
///
/// # Node Adjustment
/// The example manually adjusts specific nodes to modify their neighbors:
/// - The node at position (0, 0) has its southern and eastern neighbors removed,
///   and a southeastern neighbor added.
/// - The node at position (1, 1) has a southeastern neighbor set.
///
/// # Pathfinding
/// The A* algorithm is used to find the shortest path from the start position
/// `(0, 0)` to the goal position `(9, 9)`.
///
/// # Output
/// - If a path is found, prints the grid with the path highlighted.
/// - If an error occurs, prints the error message.
///
fn main() -> Result<(), AStarError> {
    // Define a 10x10 grid.
    // This grid uses 0 for open cells and 1 for blocked cells.
    let grid = vec![
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

    // Convert the grid to a HashMap of `Node` objects.
    // Each cell in the grid is represented by a `Node` object.
    let mut nodes = Node::grid_to_nodes(&grid);

    // Manually adjust specific nodes if needed.
    let start_position = (0, 0);
    if let Some(node) = nodes.get_mut(&start_position) {
        // Removes the southern and eastern neighbors.
        node.remove_neighbor(Direction::South);
        node.remove_neighbor(Direction::East);
        // Adds a southeastern neighbor.
        node.set_neighbor(
            Direction::SouthEast,
            Some((start_position.0 + 1, start_position.1 + 1)),
        );
    }

    // Uncomment if you want to block the node at position (9, 9).
    /* if let Some(node) = nodes.get_mut(&(9, 9)) {
        node.set_blocked(true)
    }*/

    // Manually adjust specific nodes if needed.
    let adjacent_position = (1, 1);
    if let Some(node) = nodes.get_mut(&adjacent_position) {
        // Sets the southeastern neighbor for this node.
        node.set_neighbor(
            Direction::SouthEast,
            Some((adjacent_position.0 + 1, adjacent_position.1 + 1)),
        );
    }

    // Uncomment if you want to print the directions of all nodes.
    /* for ((x, y), node) in nodes.iter() {
         println!("Node ({}, {}): {:?}", x, y, node.get_directions());
    }*/

    // Uncomment if you want to see the modified directions of the node at (1, 1).
    /* if let Some(node) = nodes.get(&(adjacent_position)) {
        println!(
            "Node ({}, {}): {:?}",
            adjacent_position.0,
            adjacent_position.1,
            node.get_directions()
        );
    }*/

    // Initialize AStar with the nodes map.
    let mut astar = AStar::new(nodes);

    // Example: Find the shortest path from (0, 0) to (9, 9).
    let start = (0, 0);
    let goal = (9, 9);
    let result = astar.find_shortest_path(start, goal);

    // Handle the result of the A* algorithm.
    match result {
        Ok(path) => {
            // Print the grid with the found path.
            Node::print_grid(&grid, &path);
            println!("Path found: {:?}", path);
        }
        Err(e) => {
            // Print an error message using the Display implementation of AStarError.
            println!("An error occurred: {}", e);
        }
    }
    Ok(())
}
