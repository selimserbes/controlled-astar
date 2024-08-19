use controlled_astar::astar::AStar;
use controlled_astar::node::{Direction, Node};

fn main() {
    // Define a 10x10 grid.
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

    // Convert the matrix to a HashMap of Node objects.
    let nodes = Node::matrix_to_nodes(&matrix);

    // Uncomment this block if you want to manually modify specific nodes.
    /*
    let position = (0, 0);

    if let Some(node) = nodes.get_mut(&position) {
        node.remove_neighbor(Direction::West);
        node.set_neighbor(Direction::SouthEast, Some((position.0 + 1, position.1 + 1)));
    }
    */

    // Print the directions of all nodes.
    for ((x, y), node) in nodes.iter() {
        println!("Node ({}, {}): {:?}", x, y, node.get_directions());
    }

    // Print the directions of the specific node at position (0, 0).
    if let Some(node) = nodes.get(&(0, 0)) {
        println!("Node ({}, {}): {:?}", 0, 0, node.get_directions());
    }

    // Initialize AStar with the nodes map.
    let mut astar = AStar::new(nodes);

    // Example: Find the shortest path from (0, 0) to (9, 9).
    let start = (0, 0);
    let goal = (9, 9);
    if let Some(path) = astar.find_shortest_path(start, goal) {
        println!(
            "Path found from ({}, {}) to ({}, {}): {:?}",
            start.0, start.1, goal.0, goal.1, path
        );
    } else {
        println!(
            "No path found from ({}, {}) to ({}, {}).",
            start.0, start.1, goal.0, goal.1
        );
    }

    // Example: Find the shortest path from (1, 1) to (1, 9).
    let start = (1, 1);
    let goal = (1, 9);
    if let Some(path) = astar.find_shortest_path(start, goal) {
        println!(
            "Path found from ({}, {}) to ({}, {}): {:?}",
            start.0, start.1, goal.0, goal.1, path
        );
    } else {
        println!(
            "No path found from ({}, {}) to ({}, {}).",
            start.0, start.1, goal.0, goal.1
        );
    }
}
