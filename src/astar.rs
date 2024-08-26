use crate::node::Node;
use crate::priority_queue::{PriorityQueue, State};
use std::collections::HashMap;
use std::fmt;

/// Error types that can occur during A* pathfinding.
pub enum AStarError {
    StartNodeBlocked,
    GoalNodeBlocked,
    NodeNotFound,
    PathNotFound,
}

/// Structure implementing the A* algorithm.
#[derive(Debug)]
pub struct AStar {
    nodes: HashMap<(usize, usize), Node>,
    open_set: PriorityQueue,
    came_from: HashMap<(usize, usize), (usize, usize)>,
    g_score: HashMap<(usize, usize), usize>,
    f_score: HashMap<(usize, usize), usize>,
}

impl AStar {
    /// Creates a new `AStar` instance with the provided nodes.
    ///
    /// # Parameters
    /// - `nodes`: A map of nodes where the keys are positions (x, y) and the values are `Node` objects.
    ///
    /// # Returns
    /// A new `AStar` instance initialized with the provided nodes.
    ///
    /// # Example
    /// ```rust
    /// let nodes = HashMap::new();
    /// let astar = AStar::new(nodes);
    /// ```
    pub fn new(nodes: HashMap<(usize, usize), Node>) -> Self {
        AStar {
            nodes,
            open_set: PriorityQueue::new(),
            came_from: HashMap::new(),
            g_score: HashMap::new(),
            f_score: HashMap::new(),
        }
    }

    /// Calculates the Manhattan distance between two points.
    ///
    /// The Manhattan distance is the sum of the absolute differences of their Cartesian coordinates.
    ///
    /// # Parameters
    /// - `start`: The starting point as a tuple (x, y).
    /// - `goal`: The goal point as a tuple (x, y).
    ///
    /// # Returns
    /// The Manhattan distance between the start and goal points as a `usize`.
    fn manhattan_distance(start: (usize, usize), goal: (usize, usize)) -> usize {
        let (x1, y1) = start;
        let (x2, y2) = goal;

        // Compute the absolute differences in x and y coordinates and sum them
        (x1 as isize - x2 as isize).abs() as usize + (y1 as isize - y2 as isize).abs() as usize
    }

    /// Initializes the `g_score` and `f_score` maps for the A* algorithm.
    ///
    /// The `g_score` map stores the cost of the shortest path from the start node to each node.
    /// The `f_score` map estimates the total cost of the shortest path through each node.
    ///
    /// # Parameters
    /// - `start`: The starting point as a tuple (x, y).
    /// - `goal`: The goal point as a tuple (x, y).
    ///
    /// # Returns
    /// A tuple containing the initialized `g_score` and `f_score` maps.
    fn initialize_scores(
        start: (usize, usize),
        goal: (usize, usize),
    ) -> (
        HashMap<(usize, usize), usize>,
        HashMap<(usize, usize), usize>,
    ) {
        let mut g_score = HashMap::new();
        let mut f_score = HashMap::new();

        // Set the starting point's g_score to 0
        g_score.insert(start, 0);

        // Compute the initial f_score as the Manhattan distance from the start to the goal
        f_score.insert(start, Self::manhattan_distance(start, goal));

        (g_score, f_score)
    }

    /// Reconstructs the path from the start node to the goal node using the `came_from` map.
    ///
    /// The path is reconstructed by tracing back from the goal node to the start node.
    ///
    /// # Parameters
    /// - `came_from`: A map indicating the parent of each node.
    /// - `start`: The starting point as a tuple (x, y).
    /// - `goal`: The goal point as a tuple (x, y).
    ///
    /// # Returns
    /// A vector of tuples representing the path from the start to the goal.
    fn reconstruct_path(
        came_from: HashMap<(usize, usize), (usize, usize)>,
        start: (usize, usize),
        goal: (usize, usize),
    ) -> Vec<(usize, usize)> {
        let mut path = Vec::new();
        let mut current = goal;

        // Trace the path from the goal to the start
        while current != start {
            path.push((current.1, current.0));
            current = came_from[&current];
        }

        // Add the start point and reverse the path to get it from start to goal
        path.push((start.1, start.0));
        path.reverse();
        path
    }

    /// Finds the neighbors of the current node that are not blocked.
    ///
    /// # Parameters
    /// - `current_node`: The current node from which to find neighbors.
    ///
    /// # Returns
    /// A vector of positions representing the neighbors of the current node.
    fn find_neighbors(&self, current_node: &Node) -> Vec<(usize, usize)> {
        current_node
            .neighbors
            .values()
            .filter_map(|&neighbor_pos| {
                neighbor_pos.and_then(|pos| {
                    // Check if the neighbor is not blocked and exists in the nodes map
                    if let Some(neighbor_node) = self.nodes.get(&pos) {
                        if !neighbor_node.is_blocked {
                            return Some(pos);
                        }
                    }
                    None
                })
            })
            .collect()
    }

    /// Calculates and updates the `g_score` and `f_score` for a neighbor node.
    ///
    /// The `g_score` represents the cost of the shortest path from the start node to the neighbor node.
    /// The `f_score` is the estimated total cost from the start node to the goal node through the neighbor node.
    ///
    /// # Parameters
    /// - `current_position`: The position of the current node.
    /// - `neighbor_pos`: The position of the neighbor node.
    /// - `goal`: The goal point as a tuple (x, y).
    ///
    /// # Returns
    /// The calculated `f_score` for the neighbor node.
    fn calculate_scores(
        &mut self,
        current_position: (usize, usize),
        neighbor_pos: (usize, usize),
        goal: (usize, usize),
    ) -> usize {
        // Calculate the tentative g_score for the neighbor
        let tentative_g_score = self.g_score[&current_position] + 1;

        // Check if this path to the neighbor is better than any previously recorded path
        if tentative_g_score < *self.g_score.get(&neighbor_pos).unwrap_or(&usize::MAX) {
            // Update the path and scores
            self.came_from.insert(neighbor_pos, current_position);
            self.g_score.insert(neighbor_pos, tentative_g_score);
            let f_score_value = tentative_g_score + Self::manhattan_distance(neighbor_pos, goal);
            self.f_score.insert(neighbor_pos, f_score_value);
            return f_score_value;
        }
        usize::MAX
    }

    /// Checks if the current node has reached the goal.
    ///
    /// # Parameters
    /// - `current_position`: The position of the current node.
    /// - `goal`: The goal point as a tuple (x, y).
    ///
    /// # Returns
    /// `true` if the current position is the goal, otherwise `false`.
    fn is_goal_reached(&self, current_position: (usize, usize), goal: (usize, usize)) -> bool {
        current_position == goal
    }

    /// Processes a neighbor node by calculating its `f_score` and adding it to the open set if necessary.
    ///
    /// # Parameters
    /// - `current_position`: The position of the current node.
    /// - `neighbor_pos`: The position of the neighbor node.
    /// - `goal`: The goal point as a tuple (x, y).
    fn process_neighbor(
        &mut self,
        current_position: (usize, usize),
        neighbor_pos: (usize, usize),
        goal: (usize, usize),
    ) {
        // Calculate the f_score for the neighbor
        let f_score_value = self.calculate_scores(current_position, neighbor_pos, goal);

        // If the f_score is valid, add the neighbor to the open set
        if f_score_value != usize::MAX {
            self.open_set.push(State {
                cost: f_score_value,
                position: neighbor_pos,
            });
        }
    }

    /// Finds the shortest path from start to goal using the A* algorithm.
    ///
    /// # Parameters
    /// - `start`: The starting point as a tuple (x, y).
    /// - `goal`: The goal point as a tuple (x, y).
    ///
    /// # Returns
    /// A `Result<Option<Vec<(usize, usize)>>, AStarError>` containing the path from the start to the goal if found,
    /// or an `AStarError` if no path is found or if an error occurs.
    ///
    /// # Example
    /// ```rust
    /// let mut astar = AStar::new(nodes);
    /// let path = astar.find_shortest_path((0, 0), (5, 5));
    /// ```
    pub fn find_shortest_path(
        &mut self,
        start: (usize, usize),
        goal: (usize, usize),
    ) -> Result<Option<Vec<(usize, usize)>>, AStarError> {
        // Check if the start node exists and is not blocked
        if self.nodes.get(&start).map_or(true, |node| node.is_blocked) {
            return Err(AStarError::StartNodeBlocked);
        }
        if self.nodes.get(&goal).map_or(true, |node| node.is_blocked) {
            return Err(AStarError::GoalNodeBlocked);
        }

        // Reset the open set and clear previous scores and path information
        self.open_set = PriorityQueue::new(); // Reset the open set
        self.came_from.clear(); // Clear the `came_from` map
        self.g_score.clear(); // Clear the `g_score` map
        self.f_score.clear(); // Clear the `f_score` map

        // Initialize scores for the start and goal
        let (g_score, f_score) = Self::initialize_scores((start.1, start.0), (goal.1, goal.0));
        self.g_score = g_score;
        self.f_score = f_score;

        // Add the start position to the open set
        self.open_set.push(State {
            cost: self.f_score[&(start.1, start.0)],
            position: (start.1, start.0),
        });

        // Main loop of the A* algorithm
        while let Some(current_state) = self.open_set.pop() {
            let current_position = current_state.position;

            // Check if the goal has been reached
            if self.is_goal_reached(current_position, (goal.1, goal.0)) {
                // Reconstruct and return the path
                return Ok(Some(Self::reconstruct_path(
                    self.came_from.clone(),
                    (start.1, start.0),
                    (goal.1, goal.0),
                )));
            }

            // Process each neighbor of the current node
            if let Some(current_node) = self.nodes.get(&current_position) {
                for neighbor_pos in self.find_neighbors(current_node) {
                    self.process_neighbor(current_position, neighbor_pos, (goal.1, goal.0));
                }
            }
        }

        Err(AStarError::PathNotFound)
    }
}

impl fmt::Display for AStarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match *self {
            AStarError::StartNodeBlocked => "The start node is blocked!",
            AStarError::GoalNodeBlocked => "The goal node is blocked!",
            AStarError::NodeNotFound => "One of the nodes is not found!",
            AStarError::PathNotFound => "No path could be found!",
        };
        write!(f, "{}", message)
    }
}

impl fmt::Debug for AStarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
