#[cfg(test)]
mod tests {
    use controlled_astar::{Direction, Node};

    #[test]
    fn test_node_creation() {
        let node = Node::new(2, 3, false, 9, 9);

        // Node attributes
        assert_eq!(node.x, 2);
        assert_eq!(node.y, 3);
        assert_eq!(node.is_blocked, false);

        // Node neighbors
        assert_eq!(node.neighbors[&Direction::North], Some((2, 2)));
        assert_eq!(node.neighbors[&Direction::South], Some((2, 4)));
        assert_eq!(node.neighbors[&Direction::West], Some((1, 3)));
        assert_eq!(node.neighbors[&Direction::East], Some((3, 3)));
    }

    #[test]
    fn test_node_creation_edges() {
        let node_top_left = Node::new(0, 0, false, 9, 9);
        let node_bottom_right = Node::new(9, 9, false, 9, 9);

        // Top-left corner node neighbors
        assert_eq!(node_top_left.neighbors.get(&Direction::North), None);
        assert_eq!(node_top_left.neighbors.get(&Direction::West), None);
        assert_eq!(node_top_left.neighbors[&Direction::South], Some((0, 1)));
        assert_eq!(node_top_left.neighbors[&Direction::East], Some((1, 0)));

        // Bottom-right corner node neighbors
        assert_eq!(node_bottom_right.neighbors.get(&Direction::South), None);
        assert_eq!(node_bottom_right.neighbors.get(&Direction::East), None);
        assert_eq!(node_bottom_right.neighbors[&Direction::North], Some((9, 8)));
        assert_eq!(node_bottom_right.neighbors[&Direction::West], Some((8, 9)));
    }

    #[test]
    fn test_set_neighbor() {
        let mut node = Node::new(2, 2, false, 9, 9);

        // Set a neighbor manually
        node.set_neighbor(Direction::North, Some((2, 1)));
        assert_eq!(node.neighbors[&Direction::North], Some((2, 1)));

        // Set a neighbor to None (no neighbor)
        node.set_neighbor(Direction::South, None);
        assert_eq!(node.neighbors.get(&Direction::South), Some(&None));
    }

    #[test]
    fn test_remove_neighbor() {
        let mut node = Node::new(2, 2, false, 9, 9);

        // Initially has neighbors
        assert!(node.neighbors.contains_key(&Direction::North));

        // Remove a neighbor
        node.remove_neighbor(Direction::North);
        assert!(!node.neighbors.contains_key(&Direction::North));
    }

    #[test]
    fn test_set_blocked() {
        let mut node = Node::new(2, 2, false, 9, 9);

        // Initially not blocked
        assert!(!node.is_blocked);

        // Set node as blocked
        node.set_blocked(true);
        assert!(node.is_blocked);
    }

    #[test]
    fn test_get_directions() {
        let node = Node::new(2, 2, false, 9, 9);
        let directions = node.get_directions();

        // Should have 4 directions (North, South, West, East)
        assert_eq!(directions.len(), 4);
        assert!(directions.contains(&Direction::North));
        assert!(directions.contains(&Direction::South));
        assert!(directions.contains(&Direction::West));
        assert!(directions.contains(&Direction::East));
    }

    #[test]
    fn test_matrix_to_nodes() {
        let matrix = vec![vec![0, 1, 0], vec![1, 0, 1], vec![0, 1, 0]];
        let nodes = Node::matrix_to_nodes(&matrix);

        // Check if the matrix was converted correctly
        assert_eq!(nodes.len(), 9);

        // Check specific nodes
        assert_eq!(nodes[&(0, 0)].is_blocked, false);
        assert_eq!(nodes[&(0, 1)].is_blocked, true);
        assert_eq!(nodes[&(1, 1)].is_blocked, false);
    }
}
