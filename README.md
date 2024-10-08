# controlled_astar

[`controlled_astar`](https://crates.io/crates/controlled_astar) is a Rust library that provides an enhanced A\* pathfinding algorithm with controllable node directions and block statuses. This library simulates real-world constraints, such as restricted directions and impassable obstacles, making it a powerful tool for complex pathfinding scenarios.

## Overview

`controlled_astar` extends the classic A\* pathfinding algorithm by allowing fine-grained control over node directions and blockages. This feature enables you to model scenarios where nodes have restricted movement options or are completely impassable. It is ideal for use in applications that require sophisticated pathfinding capabilities, such as game development, robotics, and simulation.

## Features

- **Customizable Node Directions:** Adjust which directions nodes can move in, allowing for more precise control over pathfinding.
- **Blockable Nodes:** Set nodes as blocked to prevent the pathfinding algorithm from considering them.
- **Enhanced Pathfinding:** Uses the A\* algorithm to find the shortest path between nodes in a grid with customizable constraints.

## Installation

Add `controlled_astar` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
controlled_astar = "1.0.1"
```

## Usage

### Coordinate System and Example Map

The following 10x10 grid map is used to demonstrate the coordinate system and movement directions:

```
    X → +
    0 1 2 3 4 5 6 7 8 9
    +---------------------+
Y 0 | . . . . # . . . . . |
↓ 1 | . . # . # . # # # . |
+ 2 | . . . . . . . . # . |
  3 | . . . # # # . . . . |
  4 | # # . . . . . . . . |
  5 | . . . # # . . . # . |
  6 | . # . . . . . . # . |
  7 | . # # # . # . # . . |
  8 | . . . . . . . . . . |
  9 | . # . # # . . # # . |
    +---------------------+
```

#### Coordinate Directions:

- X → Move Right (Increasing X Coordinate)
- X ← Move Left (Decreasing X Coordinate)
- Y ↑ Move Up (Decreasing Y Coordinate)
- Y ↓ Move Down (Increasing Y Coordinate)

### Key Concepts

- **`AStar::new(nodes: HashMap<(usize, usize), Node>) -> AStar`**: Creates a new A\* algorithm instance using the provided node map. This initializes the pathfinding algorithm and prepares it to find the shortest path between nodes.

- **`AStar::find_shortest_path(start: (usize, usize), goal: (usize, usize)) -> Result<Option<Vec<(usize, usize)>>, AStarError>`**: Finds the shortest path between the specified start and goal positions. Returns a vector of cells representing the path if successful, or an error if the pathfinding fails.

- **`Node::grid_to_nodes(grid: &[Vec<i32>]) -> HashMap<(usize, usize), Node>`**: Converts a 2D grid (with open cells and blocked cells) into a `HashMap` of `Node` objects. Each cell in the grid is represented as a `Node`, allowing for pathfinding operations to be performed.

- **`Node::print_grid(grid: &[Vec<i32>], path: &Option<Vec<(usize, usize)>>)`**: Prints a visual representation of the grid with the given path highlighted. This function helps to visualize the pathfinding result on the grid.

- **`Node::remove_neighbor(Direction)`**: Removes a neighbor in the specified direction (e.g., North, South, East, West, etc.) from a node. This is used to restrict movement options in that direction.

- **`Node::set_neighbor(Direction, Option<(usize, usize)>)`**: Sets a neighbor for a node in the specified direction. The neighbor is indicated by its position `(x, y)`.

- **`Node::get_directions()`**: Returns a list of directions that are currently available from the node. This is useful for debugging or visualizing the movement options for a node.

- **`Node::set_blocked(bool)`**: Marks the node as blocked or unblocked. A blocked node will not be considered in pathfinding calculations, effectively acting as an obstacle.

### Basic Example

Here's a basic example of how to use the `controlled_astar` library:

```rust
// # Controlled A* Pathfinding Example
//
// This example demonstrates how to use the A* pathfinding algorithm to find
// the shortest path on a 10x10 grid. The grid consists of open cells (0)
// and blocked cells (1). Users can manually adjust the neighbors and block
// statuses of specific nodes and find the shortest path from a start point to
// a goal point.
//
// The example includes:
// - Defining a 10x10 grid with open and blocked cells
// - Converting the grid into a map of Node objects
// - Manually adjusting neighbors and blocking nodes
// - Using the A* algorithm to find the shortest path from a start to a goal
// - Printing the grid with the found path and handling any errors

use controlled_astar::{AStar, AStarError, Direction, Node};

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
```

## Documentation

For detailed API documentation and usage examples, visit the [Documentation](https://docs.rs/controlled_astar).

## Contributing

Contributions are welcome! If you'd like to contribute to controlled_astar, please fork the repository and submit a pull request with your changes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
