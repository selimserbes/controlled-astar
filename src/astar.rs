use crate::node::Node;
use crate::priority_queue::{PriorityQueue, State};
use std::collections::HashMap;

#[derive(Debug)]
pub struct AStar {
    nodes: HashMap<(usize, usize), Node>,
    open_set: PriorityQueue,
    came_from: HashMap<(usize, usize), (usize, usize)>,
    g_score: HashMap<(usize, usize), usize>,
    f_score: HashMap<(usize, usize), usize>,
}

impl AStar {
    // Creates a new AStar instance
    pub fn new(nodes: HashMap<(usize, usize), Node>) -> Self {
        AStar {
            nodes,
            open_set: PriorityQueue::new(),
            came_from: HashMap::new(),
            g_score: HashMap::new(),
            f_score: HashMap::new(),
        }
    }

    // Calculates Manhattan distance between two points
    fn manhattan_distance(start: (usize, usize), goal: (usize, usize)) -> usize {
        let (x1, y1) = start;
        let (x2, y2) = goal;

        (x1 as isize - x2 as isize).abs() as usize + (y1 as isize - y2 as isize).abs() as usize
    }

    // Initializes the g_score and f_score maps
    fn initialize_scores(
        start: (usize, usize),
        goal: (usize, usize),
    ) -> (
        HashMap<(usize, usize), usize>,
        HashMap<(usize, usize), usize>,
    ) {
        let mut g_score = HashMap::new();
        let mut f_score = HashMap::new();

        g_score.insert(start, 0);
        f_score.insert(start, Self::manhattan_distance(start, goal));

        (g_score, f_score)
    }

    // Reconstructs the path from start to goal
    fn reconstruct_path(
        came_from: HashMap<(usize, usize), (usize, usize)>,
        start: (usize, usize),
        goal: (usize, usize),
    ) -> Vec<(usize, usize)> {
        let mut path = Vec::new();
        let mut current = goal;

        while current != start {
            path.push((current.1, current.0));
            current = came_from[&current];
        }

        path.push((start.1, start.0));
        path.reverse();
        path
    }

    // Finds the neighbors of a given node (excluding blocked nodes)
    fn find_neighbors(&self, current_node: &Node) -> Vec<(usize, usize)> {
        current_node
            .neighbors
            .values()
            .filter_map(|&neighbor_pos| {
                neighbor_pos.and_then(|pos| {
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

    // Calculates and updates g_score and f_score for a neighbor node
    fn calculate_scores(
        &mut self,
        current_position: (usize, usize),
        neighbor_pos: (usize, usize),
        goal: (usize, usize),
    ) -> usize {
        let tentative_g_score = self.g_score[&current_position] + 1;

        if tentative_g_score < *self.g_score.get(&neighbor_pos).unwrap_or(&usize::MAX) {
            self.came_from.insert(neighbor_pos, current_position);
            self.g_score.insert(neighbor_pos, tentative_g_score);
            let f_score_value = tentative_g_score + Self::manhattan_distance(neighbor_pos, goal);
            self.f_score.insert(neighbor_pos, f_score_value);
            return f_score_value;
        }
        usize::MAX
    }

    // Checks if the goal has been reached
    fn is_goal_reached(&self, current_position: (usize, usize), goal: (usize, usize)) -> bool {
        current_position == goal
    }

    // Processes a neighbor and adds it to the open set if necessary
    fn process_neighbor(
        &mut self,
        current_position: (usize, usize),
        neighbor_pos: (usize, usize),
        goal: (usize, usize),
    ) {
        let f_score_value = self.calculate_scores(current_position, neighbor_pos, goal);

        if f_score_value != usize::MAX {
            self.open_set.push(State {
                cost: f_score_value,
                position: neighbor_pos,
            });
        }
    }

    // Runs the A* algorithm to find the shortest path
    pub fn find_shortest_path(
        &mut self,
        start: (usize, usize),
        goal: (usize, usize),
    ) -> Option<Vec<(usize, usize)>> {
        self.open_set = PriorityQueue::new(); // Reset the open set
        self.came_from.clear(); // Clear the came_from map
        self.g_score.clear(); // Clear the g_score map
        self.f_score.clear(); // Clear the f_score map

        let (g_score, f_score) = Self::initialize_scores((start.1, start.0), (goal.1, goal.0));
        self.g_score = g_score;
        self.f_score = f_score;

        self.open_set.push(State {
            cost: self.f_score[&(start.1, start.0)],
            position: (start.1, start.0),
        });

        while let Some(current_state) = self.open_set.pop() {
            let current_position = current_state.position;

            if self.is_goal_reached(current_position, (goal.1, goal.0)) {
                return Some(Self::reconstruct_path(
                    self.came_from.clone(),
                    (start.1, start.0),
                    (goal.1, goal.0),
                ));
            }

            if let Some(current_node) = self.nodes.get(&current_position) {
                for neighbor_pos in self.find_neighbors(current_node) {
                    self.process_neighbor(current_position, neighbor_pos, (goal.1, goal.0));
                }
            }
        }

        None
    }
}
