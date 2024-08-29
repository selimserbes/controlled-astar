#[cfg(test)]
mod tests {
    use controlled_astar::{Direction, Node};

    #[test]
    fn test_node_creation() {
        // Create a new Node instance at position (2, 3) with no obstacles
        let node = Node::new(2, 3, false, 9, 9);

        // Check the node's attributes
        assert_eq!(node.x, 2); // Node's x-coordinate
        assert_eq!(node.y, 3); // Node's y-coordinate
        assert_eq!(node.is_blocked, false); // Node should not be blocked

        // Check the node's neighbors
        assert_eq!(node.neighbors[&Direction::North], Some((2, 2))); // North neighbor
        assert_eq!(node.neighbors[&Direction::South], Some((2, 4))); // South neighbor
        assert_eq!(node.neighbors[&Direction::West], Some((1, 3))); // West neighbor
        assert_eq!(node.neighbors[&Direction::East], Some((3, 3))); // East neighbor
    }

    #[test]
    fn test_node_creation_edges() {
        // Create nodes at the top-left and bottom-right corners
        let node_top_left = Node::new(0, 0, false, 9, 9);
        let node_bottom_right = Node::new(9, 9, false, 9, 9);

        // Check the neighbors for the top-left corner node
        assert_eq!(node_top_left.neighbors.get(&Direction::North), None); // No North neighbor
        assert_eq!(node_top_left.neighbors.get(&Direction::West), None); // No West neighbor
        assert_eq!(node_top_left.neighbors[&Direction::South], Some((0, 1))); // South neighbor
        assert_eq!(node_top_left.neighbors[&Direction::East], Some((1, 0))); // East neighbor

        // Check the neighbors for the bottom-right corner node
        assert_eq!(node_bottom_right.neighbors.get(&Direction::South), None); // No South neighbor
        assert_eq!(node_bottom_right.neighbors.get(&Direction::East), None); // No East neighbor
        assert_eq!(node_bottom_right.neighbors[&Direction::North], Some((9, 8))); // North neighbor
        assert_eq!(node_bottom_right.neighbors[&Direction::West], Some((8, 9)));
        // West neighbor
    }

    #[test]
    fn test_set_neighbor() {
        // Create a new Node instance
        let mut node = Node::new(2, 2, false, 9, 9);

        // Set a neighbor manually for the North direction
        node.set_neighbor(Direction::North, Some((2, 1)));
        assert_eq!(node.neighbors[&Direction::North], Some((2, 1))); // Verify the neighbor

        // Set a neighbor to None (no neighbor) for the South direction
        node.set_neighbor(Direction::South, None);
        assert_eq!(node.neighbors.get(&Direction::South), Some(&None)); // Verify the absence of neighbor
    }

    #[test]
    fn test_remove_neighbor() {
        // Create a new Node instance
        let mut node = Node::new(2, 2, false, 9, 9);

        // Ensure the node has neighbors initially
        assert!(node.neighbors.contains_key(&Direction::North));

        // Remove the North neighbor
        node.remove_neighbor(Direction::North);
        assert!(!node.neighbors.contains_key(&Direction::North)); // Verify that the neighbor is removed
    }

    #[test]
    fn test_set_blocked() {
        // Create a new Node instance
        let mut node = Node::new(2, 2, false, 9, 9);

        // Initially, the node should not be blocked
        assert!(!node.is_blocked);

        // Set the node as blocked
        node.set_blocked(true);
        assert!(node.is_blocked); // Verify that the node is now blocked
    }

    #[test]
    fn test_get_directions() {
        // Create a new Node instance
        let node = Node::new(2, 2, false, 9, 9);
        let directions = node.get_directions();

        // Check if the node has 4 valid directions
        assert_eq!(directions.len(), 4); // Should have 4 directions
        assert!(directions.contains(&Direction::North)); // North direction should be present
        assert!(directions.contains(&Direction::South)); // South direction should be present
        assert!(directions.contains(&Direction::West)); // West direction should be present
        assert!(directions.contains(&Direction::East)); // East direction should be present
    }

    #[test]
    fn test_grid_to_nodes() {
        // Define a grid representation of nodes
        let grid = vec![vec![0, 1, 0], vec![1, 0, 1], vec![0, 1, 0]];
        let nodes = Node::grid_to_nodes(&grid);

        // Check if the grid was correctly converted to nodes
        assert_eq!(nodes.len(), 9); // There should be 9 nodes

        // Check specific nodes' blocked status
        assert_eq!(nodes[&(0, 0)].is_blocked, false); // Top-left node should not be blocked
        assert_eq!(nodes[&(0, 1)].is_blocked, true); // Node (0, 1) should be blocked
        assert_eq!(nodes[&(1, 1)].is_blocked, false); // Node (1, 1) should not be blocked
    }
}
