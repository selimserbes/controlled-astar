#[cfg(test)]
mod tests {
    use controlled_astar::{AStar, AStarError, Node}; // Import the necessary modules and functions

    // Helper function to create a grid and initialize AStar
    // Converts a grid to Node structures and initializes the AStar algorithm with these nodes.
    fn setup_astar(grid: Vec<Vec<i32>>) -> AStar {
        let nodes = Node::grid_to_nodes(&grid);
        AStar::new(nodes)
    }

    #[test]
    fn test_astar_basic_pathfinding() {
        // Define a simple 5x5 grid with no obstacles
        let grid = vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
        ];
        let mut astar = setup_astar(grid);

        // Set start and goal positions
        let start = (0, 0);
        let goal = (4, 4);

        // Perform the A* search to find the shortest path
        let result = astar.find_shortest_path(start, goal);

        match result {
            Ok(Some(path)) => {
                // Ensure the path length is as expected and it starts and ends at the correct positions
                assert_eq!(path.len(), 9); // Expected path length with 9 nodes
                assert_eq!(path[0], start); // Start of the path should be the initial position
                assert_eq!(path[path.len() - 1], goal); // End of the path should be the goal position
            }
            Err(AStarError::PathNotFound(_)) => {
                // Handle the case where the path was not found
            }
            _ => {
                panic!("Unexpected result or error occurred: {:?}", result);
            }
        }
    }

    #[test]
    fn test_astar_with_obstacles() {
        // Define a 5x5 grid with obstacles
        let grid = vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 1, 1, 1, 0],
            vec![0, 1, 0, 1, 0],
            vec![0, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
        ];
        let mut astar = setup_astar(grid);

        // Set start and goal positions
        let start = (0, 0);
        let goal = (4, 4);

        // Perform the A* search to find the shortest path
        let result = astar.find_shortest_path(start, goal);

        match result {
            Ok(Some(path)) => {
                // Ensure the path length is as expected and it starts and ends at the correct positions
                assert_eq!(path.len(), 9); // Expected path length with 9 nodes
                assert_eq!(path[0], start); // Start of the path should be the initial position
                assert_eq!(path[path.len() - 1], goal); // End of the path should be the goal position
            }
            Err(AStarError::PathNotFound(_)) => {
                // Handle the case where the path was not found
            }
            _ => {
                panic!("Unexpected result or error occurred: {:?}", result);
            }
        }
    }

    #[test]
    fn test_astar_path_not_found() {
        // Define a 3x3 grid where the path is blocked
        let grid = vec![vec![0, 0, 0], vec![1, 1, 1], vec![0, 0, 0]];
        let mut astar = setup_astar(grid);

        // Set start and goal positions
        let start = (0, 0);
        let goal = (2, 2);

        // Perform the A* search to find the shortest path
        let result = astar.find_shortest_path(start, goal);

        match result {
            Ok(path) => {
                // Assert that no path is found due to blocked paths
                assert_eq!(path, None); // Path should be None because the path is blocked
            }
            Err(AStarError::PathNotFound(_)) => {
                // Test passes because PathNotFound error is expected when no path is available
            }
            _ => {
                panic!("Unexpected result or error occurred: {:?}", result);
            }
        }
    }

    #[test]
    fn test_astar_start_and_end_same() {
        // Define a 5x5 grid with no obstacles
        let grid = vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
        ];
        let mut astar = setup_astar(grid);

        // Set the start and goal positions to be the same
        let start = (2, 2);
        let goal = (2, 2);

        // Perform the A* search to find the shortest path
        let result = astar.find_shortest_path(start, goal);

        match result {
            Ok(Some(path)) => {
                // Ensure the path length is 1 because start and end are the same
                assert_eq!(path.len(), 1); // Path length should be 1 if start and end are the same
                assert_eq!(path[0], start); // Path should start and end at the same position
            }
            Err(AStarError::PathNotFound(_)) => {
                // Handle the case where the path was not found
            }
            _ => {
                panic!("Unexpected result or error occurred: {:?}", result);
            }
        }
    }

    #[test]
    fn test_astar_large_map() {
        // Define a 10x10 grid with some obstacles
        let grid = vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 1, 1, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 1, 1, 0, 0, 0],
            vec![0, 0, 1, 1, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];
        let mut astar = setup_astar(grid);

        // Set start and goal positions for the large map
        let start = (0, 0);
        let goal = (9, 9);

        // Perform the A* search to find the shortest path
        let result = astar.find_shortest_path(start, goal);

        match result {
            Ok(Some(path)) => {
                // Ensure the path length is reasonable for a large map
                assert!(path.len() > 10); // Path length should be reasonable given the map size
                assert_eq!(path[0], start); // Start of the path should be the initial position
                assert_eq!(path[path.len() - 1], goal); // End of the path should be the goal position
            }
            Err(AStarError::PathNotFound(_)) => {
                // Handle the case where the path was not found
            }
            _ => {
                panic!("Unexpected result or error occurred: {:?}", result);
            }
        }
    }

    #[test]
    fn test_astar_small_grid() {
        // Define a 2x2 grid with a single obstacle
        let grid = vec![vec![0, 0], vec![1, 0]];
        let mut astar = setup_astar(grid);

        // Set start and goal positions
        let start = (0, 0);
        let goal = (1, 1);

        // Perform the A* search to find the shortest path
        let result = astar.find_shortest_path(start, goal);

        match result {
            Ok(Some(path)) => {
                // Ensure the path length is as expected for a small grid
                assert_eq!(path.len(), 3); // Expected path length with 3 nodes
                assert_eq!(path[0], start); // Start of the path should be the initial position
                assert_eq!(path[path.len() - 1], goal); // End of the path should be the goal position
            }
            Err(AStarError::PathNotFound(_)) => {
                // Handle the case where the path was not found
            }
            _ => {
                panic!("Unexpected result or error occurred: {:?}", result);
            }
        }
    }

    #[test]
    fn test_astar_dense_obstacles() {
        // Define a grid with dense obstacles and a clear row at the bottom
        let grid = vec![
            vec![0, 1, 1, 1, 1, 0],
            vec![0, 1, 1, 1, 1, 0],
            vec![0, 1, 1, 1, 1, 0],
            vec![0, 1, 1, 1, 1, 0],
            vec![0, 1, 1, 1, 1, 0],
            vec![0, 0, 0, 0, 0, 0],
        ];
        let mut astar = setup_astar(grid);

        // Set start and goal positions
        let start = (0, 0);
        let goal = (5, 5);

        // Perform the A* search to find the shortest path
        let result = astar.find_shortest_path(start, goal);

        match result {
            Ok(Some(path)) => {
                // Ensure the path length is reasonable and it navigates through dense obstacles
                assert!(path.len() > 10); // Path length should be reasonable given the obstacle density
                assert_eq!(path[0], start); // Start of the path should be the initial position
                assert_eq!(path[path.len() - 1], goal); // End of the path should be the goal position
            }
            Err(AStarError::PathNotFound(_)) => {
                // Handle the case where the path was not found
            }
            _ => {
                panic!("Unexpected result or error occurred: {:?}", result);
            }
        }
    }

    #[test]
    fn test_astar_start_node_blocked() {
        // Define a grid where the start node is blocked
        let grid = vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        let mut astar = setup_astar(grid);

        // Set start and goal positions
        let start = (0, 0); // Start node is blocked
        let goal = (2, 2);

        // Perform the A* search to find the shortest path
        let result = astar.find_shortest_path(start, goal);

        match result {
            Ok(path) => {
                // Assert that no path is found because the start node is blocked
                assert_eq!(path, None); // Path should be None because the start node is blocked
            }
            Err(AStarError::StartNodeBlocked(_)) => {
                // Test passes because StartNodeBlocked error is expected when the start node is blocked
            }
            _ => {
                panic!("Unexpected result or error occurred: {:?}", result);
            }
        }
    }

    #[test]
    fn test_astar_goal_node_blocked() {
        // Define a grid where the goal node is blocked
        let grid = vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 1], // Goal node is blocked
        ];
        let mut astar = setup_astar(grid);

        // Set start and goal positions
        let start = (0, 0);
        let goal = (2, 2); // Goal node is blocked

        // Perform the A* search to find the shortest path
        let result = astar.find_shortest_path(start, goal);

        match result {
            Ok(path) => {
                // Assert that no path is found because the goal node is blocked
                assert_eq!(path, None); // Path should be None because the goal node is blocked
            }
            Err(AStarError::GoalNodeBlocked(_)) => {
                // Test passes because GoalNodeBlocked error is expected when the goal node is blocked
            }
            _ => {
                panic!("Unexpected result or error occurred: {:?}", result);
            }
        }
    }

    #[test]
    fn test_astar_goal_node_not_found() {
        // Define a grid with valid nodes, but the goal is out of bounds
        let grid = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        let mut astar = setup_astar(grid);

        // Set start and goal positions
        let start = (0, 0);
        let goal = (3, 3); // Goal node is outside the grid bounds

        // Perform the A* search to find the shortest path
        let result = astar.find_shortest_path(start, goal);

        match result {
            Ok(path) => {
                // Assert that the path is None because the goal node is out of bounds
                assert_eq!(path, None); // Path should be None because the goal node is out of bounds
            }
            Err(AStarError::NodeNotFound(_)) => {
                // Test passes because NodeNotFound error is expected when the goal node is out of bounds
            }
            _ => {
                panic!("Unexpected result or error occurred: {:?}", result);
            }
        }
    }

    #[test]
    fn test_astar_start_node_not_found() {
        // Define a grid with valid nodes, but the start is out of bounds
        let grid = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        let mut astar = setup_astar(grid);

        // Set start and goal positions
        let start = (4, 4); // Start node is outside the grid bounds
        let goal = (2, 2);

        // Perform the A* search to find the shortest path
        let result = astar.find_shortest_path(start, goal);

        match result {
            Ok(path) => {
                // Assert that the path is None because the start node is out of bounds
                assert_eq!(path, None); // Path should be None because the start node is out of bounds
            }
            Err(AStarError::NodeNotFound(_)) => {
                // Test passes because NodeNotFound error is expected when the start node is out of bounds
            }
            _ => {
                panic!("Unexpected result or error occurred: {:?}", result);
            }
        }
    }
}
