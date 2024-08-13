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

    fn equal(&self, current_node: &Node, end_node: &Node) -> bool {
        current_node.get_x() == end_node.get_x() && current_node.get_y() == end_node.get_y()
    }

    fn find_heuristic(&self, x: usize, y: usize, end_node: &Node) -> usize {
        (x as isize - end_node.get_x() as isize).abs() as usize
            + (y as isize - end_node.get_y() as isize).abs() as usize
    }

    fn find_g_cost(&self, mut current_node: &Node) -> usize {
        let mut g_cost = 0;

        while let Some(parent) = current_node.get_parent() {
            g_cost += self.get_hv_cost();
            current_node = parent;
        }

        g_cost
    }

    fn find_neighbours(&self, x: usize, y: usize) -> Vec<Node> {
        let mut neighbours_list = Vec::new();
        let allowed_directions = self.matrix[y][x].get_allowed_directions();

        for &(dx, dy) in allowed_directions {
            let new_x = (x as isize + dx) as usize;
            let new_y = (y as isize + dy) as usize;
            if new_x < self.matrix[0].len()
                && new_y < self.matrix.len()
                && self.matrix[new_y][new_x].get_block() != 1
            {
                let neighbour_node = Node::new(
                    new_x,
                    new_y,
                    self.matrix[new_y][new_x].get_block(),
                    self.matrix[new_y][new_x].get_allowed_directions().clone(),
                );
                neighbours_list.push(neighbour_node);
            }
        }

        neighbours_list
    }

    fn set_path(&self, end_node: &Node) -> Vec<(usize, usize)> {
        let mut path = Vec::new();
        let mut current_node = Some(end_node);

        while let Some(node) = current_node {
            path.push((node.get_x(), node.get_y()));
            current_node = node.get_parent().map(|p| &**p);
        }

        path.reverse();
        path
    }

    fn calculate_node_data(&self, neighbour: &mut Node, current_node: &Node, end_node: &Node) {
        let new_g = self.find_g_cost(current_node) + self.distance(current_node, neighbour);
        let new_h = self.find_heuristic(neighbour.get_x(), neighbour.get_y(), end_node);
        let new_f = new_g + new_h;

        neighbour.set_h(new_h);
        neighbour.set_g(new_g);
        neighbour.set_f(new_f);
    }

    fn distance(&self, node1: &Node, node2: &Node) -> usize {
        (node1.get_x() as isize - node2.get_x() as isize).abs() as usize
            + (node1.get_y() as isize - node2.get_y() as isize).abs() as usize
    }

    pub fn find_shortest_path(
        &mut self,
        point_start: (usize, usize),
        point_end: (usize, usize),
    ) -> Option<Vec<(usize, usize)>> {
        let start_node = Node::new(
            point_start.0,
            point_start.1,
            self.matrix[point_start.1][point_start.0].get_block(),
            self.matrix[point_start.1][point_start.0]
                .get_allowed_directions()
                .clone(),
        );
        let end_node = Node::new(
            point_end.0,
            point_end.1,
            self.matrix[point_end.1][point_end.0].get_block(),
            self.matrix[point_end.1][point_end.0]
                .get_allowed_directions()
                .clone(),
        );

        let mut closed_list = HashTable::new();
        let mut open_list = PriorityQueue::new(start_node.clone());

        while !open_list.is_empty() {
            let current_node = open_list.poll();
            //println!("x: {}, y: {}", current_node.get_x(), current_node.get_y());

            if self.equal(&current_node, &end_node) {
                return Some(self.set_path(&current_node));
            }

            closed_list.insert(current_node.clone());

            for mut neighbour in self.find_neighbours(current_node.get_x(), current_node.get_y()) {
                if !closed_list.search(&neighbour) {
                    if neighbour.get_parent().is_none() && !self.equal(&start_node, &neighbour) {
                        self.calculate_node_data(&mut neighbour, &current_node, &end_node);
                        neighbour.set_parent(current_node.clone());
                    }

                    if !open_list.get_queue().contains(&&neighbour) {
                        open_list.add(neighbour);
                    }
                }
            }
        }

        None
    }
}
