#[cfg(test)]
mod tests {
    use controlled_astar::{AStar, Node};
    use std::collections::HashMap;

    // Helper function to create a simple grid
    fn create_test_grid() -> HashMap<(usize, usize), Node> {
        let mut nodes = HashMap::new();
        // Create a 3x3 grid with no obstacles
        for x in 0..3 {
            for y in 0..3 {
                nodes.insert(
                    (x, y),
                    Node::new(x, y, false, 2, 2), // Initialize neighbors for a 3x3 grid
                );
            }
        }
        nodes
    }

    #[test]
    fn test_astar_empty_grid() {
        let nodes = create_test_grid();
        let mut astar = AStar::new(nodes);

        // Pathfinding from (0,0) to (2,2) on an empty grid
        let path = astar.find_shortest_path((0, 0), (2, 2));
        let expected_path = Some(vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]);

        assert_eq!(path, expected_path);
    }

    #[test]
    fn test_astar_single_node() {
        let nodes = create_test_grid();
        let mut astar = AStar::new(nodes);

        // Pathfinding from (1,1) to (1,1) should return immediately with the start as the goal
        let path = astar.find_shortest_path((1, 1), (1, 1));
        let expected_path = Some(vec![(1, 1)]);

        assert_eq!(path, expected_path);
    }

    #[test]
    fn test_astar_with_obstacles() {
        let mut nodes = create_test_grid();
        // Adding obstacles
        nodes.get_mut(&(1, 1)).unwrap().is_blocked = true;

        let mut astar = AStar::new(nodes);

        // Pathfinding from (0,0) to (2,2) with an obstacle at (1,1)
        let path = astar.find_shortest_path((0, 0), (2, 2));
        let expected_path = Some(vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]); // (1,1) is blocked, so no path should be found

        assert_eq!(path, expected_path);
    }

    #[test]
    fn test_astar_no_path_due_to_obstacles() {
        let mut nodes = create_test_grid();
        // Adding obstacles in a way that blocks all possible paths
        nodes.get_mut(&(1, 0)).unwrap().is_blocked = true;
        nodes.get_mut(&(1, 1)).unwrap().is_blocked = true;
        nodes.get_mut(&(1, 2)).unwrap().is_blocked = true;

        let mut astar = AStar::new(nodes);

        // Pathfinding from (0,0) to (2,2) with obstacles blocking all paths
        let path = astar.find_shortest_path((0, 0), (2, 2));
        let expected_path = None; // Obstacles block the path

        assert_eq!(path, expected_path);
    }

    #[test]
    fn test_astar_no_path_when_start_is_blocked() {
        let mut nodes = create_test_grid();
        // Setting the start position as blocked
        nodes.get_mut(&(0, 0)).unwrap().is_blocked = true;

        let mut astar = AStar::new(nodes);

        // Pathfinding from (0,0) to (2,2) with the start position blocked
        let path = astar.find_shortest_path((0, 0), (2, 2));
        let expected_path = None; // Start is blocked, so no path should be found

        assert_eq!(path, expected_path);
    }

    #[test]
    fn test_astar_no_path_when_goal_is_blocked() {
        let mut nodes = create_test_grid();
        // Setting the goal position as blocked
        nodes.get_mut(&(2, 2)).unwrap().is_blocked = true;

        let mut astar = AStar::new(nodes);

        // Pathfinding from (0,0) to (2,2) with the goal position blocked
        let path = astar.find_shortest_path((0, 0), (2, 2));
        let expected_path = None; // Goal is blocked, so no path should be found

        assert_eq!(path, expected_path);
    }

    #[test]
    fn test_astar_path_with_multiple_obstacles() {
        let mut nodes = create_test_grid();
        // Adding multiple obstacles
        nodes.get_mut(&(1, 1)).unwrap().is_blocked = true;
        nodes.get_mut(&(2, 1)).unwrap().is_blocked = true;

        let mut astar = AStar::new(nodes);

        // Pathfinding from (0,0) to (2,2) with obstacles
        let path = astar.find_shortest_path((0, 0), (2, 2));
        let expected_path = Some(vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]);

        assert_eq!(path, expected_path);
    }
}
