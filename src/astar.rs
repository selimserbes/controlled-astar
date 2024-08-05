#![allow(dead_code)]
use crate::hash_table::HashTable;
use crate::node::Node;
use crate::priority_queue::PriorityQueue;

pub struct AStar {
    matrix: Vec<Vec<Node>>,
    hv_cost: usize,
    diagonal_cost: usize,
}

impl AStar {
    pub fn new(matrix: Vec<Vec<Node>>, hv_cost: usize, diagonal_cost: usize) -> Self {
        AStar {
            matrix,
            hv_cost,
            diagonal_cost,
        }
    }

    fn get_hv_cost(&self) -> usize {
        self.hv_cost
    }

    fn set_hv_cost(&mut self, hv_cost: usize) {
        self.hv_cost = hv_cost;
    }

    fn get_diagonal_cost(&self) -> usize {
        self.diagonal_cost
    }

    fn set_diagonal_cost(&mut self, diagonal_cost: usize) {
        self.diagonal_cost = diagonal_cost;
    }

    fn get_matrix(&self) -> &Vec<Vec<Node>> {
        &self.matrix
    }

    fn set_matrix(&mut self, matrix: Vec<Vec<Node>>) {
        self.matrix = matrix;
    }

    fn equal(&self, current_node: &Node, end_node: &Node) -> bool {
        current_node.get_x() == end_node.get_x() && current_node.get_y() == end_node.get_y()
    }

    fn find_heuristic(&self, x: usize, y: usize, end_node: &Node) -> usize {
        (x as isize - end_node.get_x() as isize).abs() as usize
            + (y as isize - end_node.get_y() as isize).abs() as usize
    }

    fn find_neighbours(&self, x: usize, y: usize) -> Vec<Node> {
        let mut neighbours = Vec::new();
        let allowed_directions = self.get_matrix()[y][x].get_allowed_directions();
        for (dx, dy) in allowed_directions {
            let new_x = (x as isize + dx) as usize;
            let new_y = (y as isize + dy) as usize;

            if new_x < self.get_matrix()[0].len()
                && new_y < self.get_matrix().len()
                && self.get_matrix()[new_y][new_x].get_block() != 1
            {
                println!("Found neighbour at ({}, {})", new_x, new_y);
                neighbours.push(Node::new(
                    new_x,
                    new_y,
                    Some(0),
                    None,
                    Some(self.get_matrix()[new_y][new_x].clone()),
                ));
            }
        }
        neighbours
    }

    fn set_path(&self, end_node: &Node) -> Vec<(usize, usize)> {
        let mut path = Vec::new();
        let mut current_node = end_node;
        while let Some(parent) = current_node.get_parent() {
            println!(
                "Adding to path: ({}, {})",
                current_node.get_x(),
                current_node.get_y()
            );
            path.push((current_node.get_x(), current_node.get_y()));
            current_node = parent;
        }
        println!(
            "Adding start node to path: ({}, {})",
            current_node.get_x(),
            current_node.get_y()
        );
        path.push((current_node.get_x(), current_node.get_y()));
        path.reverse();
        path
    }

    fn check_parent_with_start(&self, start_node: &Node, neighbour: &Node) -> bool {
        if let Some(parent) = neighbour.get_parent() {
            start_node.get_x() == parent.get_x() && start_node.get_y() == parent.get_y()
        } else {
            false
        }
    }

    fn distance(&self, node1: &Node, node2: &Node) -> usize {
        (node1.get_x() as isize - node2.get_x() as isize).abs() as usize
            + (node1.get_y() as isize - node2.get_y() as isize).abs() as usize
    }

    fn find_g_cost(&self, current_node: &Node) -> usize {
        let mut g_cost = 0;
        let mut node = current_node;
        while let Some(parent) = node.get_parent() {
            g_cost += if self.distance(node, parent) == 1 {
                self.get_hv_cost()
            } else {
                self.get_diagonal_cost()
            };
            node = parent;
        }
        g_cost
    }

    fn calculate_node_data(
        &self,
        neighbour: &mut Node,
        current_node: &Node,
        end_node: &Node,
    ) -> bool {
        let previous_f_cost = neighbour.get_f();
        let new_g = self.find_g_cost(current_node) + self.distance(current_node, neighbour);
        let new_h = self.find_heuristic(neighbour.get_x(), neighbour.get_y(), end_node);
        let new_f = new_g + new_h;

        neighbour.set_h(new_h);
        neighbour.set_g(new_g);
        neighbour.set_f(new_f);

        println!(
            "Neighbour ({}, {}): g={}, h={}, f={}, previous_f={}",
            neighbour.get_x(),
            neighbour.get_y(),
            new_g,
            new_h,
            new_f,
            previous_f_cost
        );

        if previous_f_cost <= new_f {
            false
        } else {
            neighbour.set_parent(Box::new(current_node.clone()));
            true
        }
    }

    pub fn find_shortest_path(
        &mut self,
        point_start: (usize, usize),
        point_end: (usize, usize),
    ) -> Option<Vec<(usize, usize)>> {
        let start_node = Node::new(point_start.0, point_start.1, Some(0), None, None);
        let end_node = Node::new(point_end.0, point_end.1, Some(0), None, None);

        let mut closed_list = HashTable::new();
        let mut open_list = PriorityQueue::new();
        open_list.add(start_node.clone());

        while !open_list.is_empty() {
            let current_node = open_list.poll()?;

            if self.equal(&current_node, &end_node) {
                println!("Path found, setting path...");
                return Some(self.set_path(&current_node));
            }

            closed_list.insert(current_node.clone());
            println!(
                "Current node: ({}, {})",
                current_node.get_x(),
                current_node.get_y()
            );

            for mut neighbour in self.find_neighbours(current_node.get_x(), current_node.get_y()) {
                if closed_list.search(&neighbour) {
                    println!(
                        "Neighbour ({}, {}) is already in closed list.",
                        neighbour.get_x(),
                        neighbour.get_y()
                    );
                    continue;
                }

                let is_new_node = neighbour.get_parent().is_none();

                if is_new_node || self.calculate_node_data(&mut neighbour, &current_node, &end_node)
                {
                    println!(
                        "Updating parent for node: ({}, {})",
                        neighbour.get_x(),
                        neighbour.get_y()
                    );
                    neighbour.set_parent(Box::new(current_node.clone()));

                    if !open_list.contains(&neighbour) {
                        open_list.add(neighbour.clone()); // Klonlama
                        println!(
                            "Added to open list: ({}, {})",
                            neighbour.get_x(),
                            neighbour.get_y()
                        );
                    }
                } else {
                    println!(
                        "Node ({}, {}) not added to open list or updated.",
                        neighbour.get_x(),
                        neighbour.get_y()
                    );
                }
            }
        }

        None
    }
}
