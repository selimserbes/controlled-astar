#[cfg(test)]
mod tests {
    use controlled_astar::{AStar, AStarError, Node};
    use std::collections::HashMap;

    // Helper function to create a simple 3x3 grid with no obstacles
    fn create_test_grid() -> HashMap<(usize, usize), Node> {
        let mut nodes = HashMap::new();
        for x in 0..3 {
            for y in 0..3 {
                nodes.insert(
                    (x, y),
                    Node::new(x, y, false, 2, 2), // Initialize nodes with no obstacles
                );
            }
        }
        nodes
    }

    #[test]
    fn test_astar_empty_grid() {
        // Create a grid with no obstacles
        let nodes = create_test_grid();
        let mut astar = AStar::new(nodes);

        // Find the shortest path from (0, 0) to (2, 2)
        let result = astar.find_shortest_path((0, 0), (2, 2));

        match result {
            Ok(path) => {
                let expected_path = vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)];
                // Assert that the path is as expected
                assert_eq!(path, Some(expected_path));
            }
            Err(AStarError::PathNotFound(_)) => {
                panic!("Expected a path, but got PathNotFound error.");
            }
            Err(e) => {
                panic!("An error occurred: {:?}", e);
            }
        }
    }

    #[test]
    fn test_astar_single_node() {
        // Create a grid with no obstacles
        let nodes = create_test_grid();
        let mut astar = AStar::new(nodes);

        // Find the path from (1, 1) to (1, 1), which is the same start and end
        let result = astar.find_shortest_path((1, 1), (1, 1));

        match result {
            Ok(path) => {
                let expected_path = vec![(1, 1)];
                // Assert that the path consists of the single node
                assert_eq!(path, Some(expected_path));
            }
            Err(AStarError::PathNotFound(_)) => {
                panic!("Expected a path, but got PathNotFound error.");
            }
            Err(e) => {
                panic!("An error occurred: {:?}", e);
            }
        }
    }

    #[test]
    fn test_astar_with_obstacles() {
        // Create a grid and add an obstacle
        let mut nodes = create_test_grid();
        nodes.get_mut(&(1, 1)).unwrap().is_blocked = true;

        let mut astar = AStar::new(nodes);

        // Find the shortest path from (0, 0) to (2, 2) with an obstacle
        let result = astar.find_shortest_path((0, 0), (2, 2));

        match result {
            Ok(path) => {
                let expected_path = vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)];
                // Assert that the path avoids the obstacle at (1, 1)
                assert_eq!(path, Some(expected_path));
            }
            Err(AStarError::PathNotFound(_)) => {
                panic!("Expected a path, but got PathNotFound error.");
            }
            Err(e) => {
                panic!("An error occurred: {:?}", e);
            }
        }
    }

    #[test]
    fn test_astar_no_path_due_to_obstacles() {
        // Create a grid and add obstacles that block all paths
        let mut nodes = create_test_grid();
        nodes.get_mut(&(1, 0)).unwrap().is_blocked = true;
        nodes.get_mut(&(1, 1)).unwrap().is_blocked = true;
        nodes.get_mut(&(1, 2)).unwrap().is_blocked = true;

        let mut astar = AStar::new(nodes);

        // Find the shortest path from (0, 0) to (2, 2) with all paths blocked
        let result = astar.find_shortest_path((0, 0), (2, 2));

        match result {
            Ok(path) => {
                // Assert that no path is found due to blocked paths
                assert_eq!(path, None);
            }
            Err(AStarError::PathNotFound(_)) => {
                // Test passes because PathNotFound error is expected
            }
            Err(e) => {
                panic!("Expected PathNotFound error but got: {:?}", e);
            }
        }
    }

    #[test]
    fn test_astar_no_path_when_start_is_blocked() {
        // Create a grid and block the start position
        let mut nodes = create_test_grid();
        nodes.get_mut(&(0, 0)).unwrap().is_blocked = true;

        let mut astar = AStar::new(nodes);

        // Attempt to find a path from blocked start position
        let result = astar.find_shortest_path((0, 0), (2, 2));

        match result {
            Ok(path) => {
                // Assert that no path is found because the start node is blocked
                assert_eq!(path, None);
            }
            Err(AStarError::StartNodeBlocked(_)) => {
                // Test passes because StartNodeBlocked error is expected
            }
            Err(e) => {
                panic!("Expected StartNodeBlocked error but got: {:?}", e);
            }
        }
    }

    #[test]
    fn test_astar_no_path_when_goal_is_blocked() {
        // Create a grid and block the goal position
        let mut nodes = create_test_grid();
        nodes.get_mut(&(2, 2)).unwrap().is_blocked = true;

        let mut astar = AStar::new(nodes);

        // Attempt to find a path to a blocked goal position
        let result = astar.find_shortest_path((0, 0), (2, 2));

        match result {
            Ok(path) => {
                // Assert that no path is found because the goal node is blocked
                assert_eq!(path, None);
            }
            Err(AStarError::GoalNodeBlocked(_)) => {
                // Test passes because GoalNodeBlocked error is expected
            }
            Err(e) => {
                panic!("Expected GoalNodeBlocked error but got: {:?}", e);
            }
        }
    }

    #[test]
    fn test_astar_path_with_multiple_obstacles() {
        // Create a grid and add multiple obstacles
        let mut nodes = create_test_grid();
        nodes.get_mut(&(1, 1)).unwrap().is_blocked = true;
        nodes.get_mut(&(2, 1)).unwrap().is_blocked = true;

        let mut astar = AStar::new(nodes);

        // Find the shortest path from (0, 0) to (2, 2) with multiple obstacles
        let result = astar.find_shortest_path((0, 0), (2, 2));

        match result {
            Ok(path) => {
                let expected_path = vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)];
                // Assert that the path avoids the obstacles
                assert_eq!(path, Some(expected_path));
            }
            Err(AStarError::PathNotFound(_)) => {
                panic!("Expected a path, but got PathNotFound error.");
            }
            Err(e) => {
                panic!("An error occurred: {:?}", e);
            }
        }
    }

    #[test]
    fn test_astar_node_not_found_for_start() {
        // Create a grid and test a start node that is out of bounds
        let nodes = create_test_grid();
        let mut astar = AStar::new(nodes);

        // Start node is outside the grid
        let result = astar.find_shortest_path((10, 10), (2, 2));

        match result {
            Ok(path) => {
                // Assert that the path is None because the start node is out of bounds
                assert_eq!(path, None);
            }
            Err(AStarError::NodeNotFound(_)) => {
                // Test passes because NodeNotFound error is expected
            }
            Err(e) => {
                panic!("An error occurred: {:?}", e);
            }
        }
    }

    #[test]
    fn test_astar_node_not_found_for_goal() {
        // Create a grid and test a goal node that is out of bounds
        let nodes = create_test_grid();
        let mut astar = AStar::new(nodes);

        // Goal node is outside the grid
        let result = astar.find_shortest_path((0, 0), (10, 10));

        match result {
            Ok(path) => {
                // Assert that the path is None because the goal node is out of bounds
                assert_eq!(path, None);
            }
            Err(AStarError::NodeNotFound(_)) => {
                // Test passes because NodeNotFound error is expected
            }
            Err(e) => {
                panic!("An error occurred: {:?}", e);
            }
        }
    }
}
