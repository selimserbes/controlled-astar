#![allow(dead_code)]
use crate::hash_table::HashTable;
use crate::node::Node;
use crate::priority_queue::PriorityQueue;

pub struct AStar {
    matrix: Vec<Vec<Node>>,
    hv_cost: i32,
    diagonal_cost: i32,
}

impl AStar {
    pub fn new(matrix: Vec<Vec<Node>>, hv_cost: i32, diagonal_cost: i32) -> Self {
        AStar {
            matrix,
            hv_cost,
            diagonal_cost,
        }
    }

    fn get_hv_cost(&self) -> i32 {
        self.hv_cost
    }

    fn set_hv_cost(&mut self, hv_cost: i32) {
        self.hv_cost = hv_cost;
    }

    fn get_diagonal_cost(&self) -> i32 {
        self.diagonal_cost
    }

    fn set_diagonal_cost(&mut self, diagonal_cost: i32) {
        self.diagonal_cost = diagonal_cost;
    }

    fn equal(&self, current_node: &Node, end_node: &Node) -> bool {
        current_node.x() == end_node.x() && current_node.y() == end_node.y()
    }

    // Manhattan Heuristic
    fn find_heuristic(&self, x: i32, y: i32, end_node: &Node) -> i32 {
        (x - end_node.x()).abs() + (y - end_node.y()).abs()
    }

    /*
    // Octile Heuristic
    fn find_heuristic(&self, x: i32, y: i32, end_node: &Node) -> f64 {
        let dx = (x - end_node.x()).abs();
        let dy = (y - end_node.y()).abs();
        dx.max(dy) + (2f64.sqrt() - 1.0) * dx.min(dy) as f64
    }
    */

    /*
    // Chebyshev Heuristic
    fn find_heuristic(&self, x: i32, y: i32, end_node: &Node) -> i32 {
        (x - end_node.x()).abs().max((y - end_node.y()).abs())
    }
    */

    /*
    // Euclidean Heuristic
    fn find_heuristic(&self, x: i32, y: i32, end_node: &Node) -> f64 {
        let dx = (x - end_node.x()).pow(2);
        let dy = (y - end_node.y()).pow(2);
        ((dx + dy) as f64).sqrt()
    }
    */

    fn find_g_cost(&self, mut current_node: Option<&Node>) -> i32 {
        let mut g_cost = 0;
        while let Some(node) = current_node {
            if let Some(parent) = node.parent() {
                g_cost += self.get_hv_cost();
                current_node = Some(parent);
            } else {
                current_node = None;
            }
        }
        g_cost
    }

    fn find_neighbours(&self, x: i32, y: i32) -> Vec<Node> {
        let mut neighbours_list = Vec::new();

        // get directions from node
        if let Some(current_node) = self
            .matrix
            .get(y as usize)
            .and_then(|row| row.get(x as usize))
        {
            let allowed_directions = current_node.allowed_directions();
            for (dx, dy) in allowed_directions {
                let new_x = x + dx;
                let new_y = y + dy;
                if new_x >= 0
                    && new_x < self.matrix[0].len() as i32
                    && new_y >= 0
                    && new_y < self.matrix.len() as i32
                    && self.matrix[new_y as usize][new_x as usize].block() != 1
                {
                    let neighbour_node = self.matrix[new_y as usize][new_x as usize].clone();
                    neighbours_list.push(neighbour_node);
                }
            }
        }

        neighbours_list
    }

    fn set_path(&self, mut end_node: &Node) -> Vec<Vec<i32>> {
        let mut path = Vec::new();

        while let Some(parent) = end_node.parent() {
            path.push(vec![end_node.x(), end_node.y()]);
            end_node = parent;
        }

        path.reverse();
        println!("{:?}", path);
        path
    }

    fn check_parent_with_start(&self, start_node: &Node, neighbour: &Node) -> bool {
        if let Some(parent) = neighbour.parent() {
            return start_node.x() == parent.x() && start_node.y() == parent.y();
        }
        false
    }

    fn calculate_node_data(
        &self,
        neighbour: &mut Node,
        current_node: &Node,
        end_node: &Node,
    ) -> bool {
        let previous_f_cost = neighbour.f();

        // Calculate the new g cost
        let new_g = self.find_g_cost(Some(current_node)) + self.distance(current_node, neighbour);
        // Calculate the new h cost
        let new_h = self.find_heuristic(neighbour.x(), neighbour.y(), end_node);
        // Calculate the new f cost
        let new_f = new_g + new_h;

        // Set the new values for the node
        neighbour.set_h(new_h);
        neighbour.set_g(new_g);
        neighbour.set_f(new_f);
        println!("hello : {:?}", neighbour.parent());
        // If the previous f cost is lower, do not update
        if previous_f_cost <= new_f {
            return false;
        }

        // Make the updates
        neighbour.set_parent(Some(Box::new(current_node.clone())));

        true
    }

    // Manhattan distance
    fn distance(&self, node1: &Node, node2: &Node) -> i32 {
        (node1.x() - node2.x()).abs() + (node2.y() - node1.y()).abs()
    }

    /*
    // Octile distance
    fn distance(&self, node1: &Node, node2: &Node) -> f64 {
        let dx = (node1.x() - node2.x()).abs();
        let dy = (node1.y() - node2.y()).abs();
        dx.max(dy) + (2f64.sqrt() - 1.0) * dx.min(dy) as f64
    }
    */

    /*
    // Chebyshev distance
    fn distance(&self, node1: &Node, node2: &Node) -> i32 {
        (node1.x() - node2.x()).abs().max((node1.y() - node2.y()).abs())
    }
    */

    /*
    // Euclidean distance
    fn distance(&self, node1: &Node, node2: &Node) -> f64 {
        let dx = (node1.x() - node2.x()).pow(2);
        let dy = (node1.y() - node2.y()).pow(2);
        ((dx + dy) as f64).sqrt()
    }
    */

    pub fn find_shortest_path(
        &mut self,
        point_start: (i32, i32),
        point_end: (i32, i32),
    ) -> Option<Vec<Vec<i32>>> {
        let start_node = Node::new(point_start.0, point_start.1, 0, None, None);
        let end_node = Node::new(point_end.0, point_end.1, 0, None, None);
        let mut closed_list = HashTable::new(900000, 0.75);
        let mut open_list = PriorityQueue::new();

        // Add start node to the open list with its initial f value
        open_list.add(start_node.clone(), start_node.f());
        println!("start node: {:?}", start_node);

        while !open_list.is_empty() {
            let current_node = open_list.poll().expect("Expected a node to be polled");
            println!("x: {}, y: {}", current_node.x(), current_node.y());

            if self.equal(&current_node, &end_node) {
                println!("{:?}", current_node);
                return Some(self.set_path(&current_node));
            }

            closed_list.insert(current_node.clone());

            for mut neighbour in self.find_neighbours(current_node.x(), current_node.y()) {
                if !closed_list.contains(&neighbour) {
                    if neighbour.parent().is_none() && !self.equal(&start_node, &neighbour) {
                        self.calculate_node_data(&mut neighbour, &current_node, &end_node);
                        neighbour.set_parent(Some(Box::new(current_node.clone())));
                    } else if current_node.f() < neighbour.f()
                        && !self.check_parent_with_start(&start_node, &neighbour)
                    {
                        if self.calculate_node_data(&mut neighbour, &current_node, &end_node) {
                            neighbour.set_parent(Some(Box::new(current_node.clone())));
                            println!("{:?}", neighbour.parent())
                        }
                    }

                    // Add the neighbour to the open list with its f value
                    if !open_list.get_queue().contains(&neighbour) {
                        open_list.add(neighbour.clone(), neighbour.f());
                    }
                }
            }
        }
        None
    }
}
