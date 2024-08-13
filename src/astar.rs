use crate::hash_table::HashTable;
use crate::node::Node;
use crate::priority_queue::PriorityQueue;

pub struct AStar {
    matrix: Vec<Vec<Node>>,
    default_hv_cost: i32,
    default_diagonal_cost: i32,
}

impl AStar {
    pub fn new(matrix: Vec<Vec<Node>>, hv_cost: i32, diagonal_cost: i32) -> Self {
        AStar {
            matrix,
            default_hv_cost: hv_cost,
            default_diagonal_cost: diagonal_cost,
        }
    }

    pub fn get_hv_cost(&self) -> i32 {
        self.default_hv_cost
    }

    pub fn set_hv_cost(&mut self, hv_cost: i32) {
        self.default_hv_cost = hv_cost;
    }

    pub fn get_diagonal_cost(&self) -> i32 {
        self.default_diagonal_cost
    }

    pub fn set_diagonal_cost(&mut self, diagonal_cost: i32) {
        self.default_diagonal_cost = diagonal_cost;
    }

    pub fn get_matrix(&self) -> &Vec<Vec<Node>> {
        &self.matrix
    }

    pub fn set_matrix(&mut self, matrix: Vec<Vec<Node>>) {
        self.matrix = matrix;
    }

    fn equal(&self, current_node: &Node, end_node: &Node) -> bool {
        current_node.get_x() == end_node.get_x() && current_node.get_y() == end_node.get_y()
    }

    fn find_heuristic(&self, x: i32, y: i32, end_node: &Node) -> i32 {
        (x - end_node.get_x()).abs() + (y - end_node.get_y()).abs()
    }

    fn find_g_cost(&self, mut current_node: &Node) -> i32 {
        let mut g_cost = 0;

        while let Some(parent) = current_node.get_parent() {
            g_cost += self.get_hv_cost();
            current_node = parent;
        }

        g_cost
    }

    fn find_neighbours(&self, x: i32, y: i32) -> Vec<Node> {
        let mut neighbours_list = Vec::new();
        let allowed_directions = self.matrix[y as usize][x as usize].get_allowed_directions();

        for (dx, dy) in allowed_directions {
            let new_x = x + dx;
            let new_y = y + dy;

            if new_x >= 0
                && new_x < self.matrix[0].len() as i32
                && new_y >= 0
                && new_y < self.matrix.len() as i32
            {
                let block = self.matrix[new_y as usize][new_x as usize].get_block();
                if block != 1 {
                    let neighbour_node = Node::new(new_x, new_y, block, None, None);
                    neighbours_list.push(neighbour_node);
                }
            }
        }

        neighbours_list
    }

    fn set_path(&self, end_node: &Node) -> Vec<(i32, i32)> {
        let mut path = Vec::new();
        let mut current_node = Some(end_node);

        while let Some(node) = current_node {
            path.push((node.get_x(), node.get_y()));
            current_node = node.get_parent();
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

    fn distance(&self, node1: &Node, node2: &Node) -> i32 {
        (node1.get_x() - node2.get_x()).abs() + (node1.get_y() - node2.get_y()).abs()
    }

    fn find_shortest_path(
        &mut self,
        point_start: (i32, i32),
        point_end: (i32, i32),
    ) -> Option<Vec<(i32, i32)>> {
        let matrix = self.get_matrix();
        let start_node = Node::new(point_start.0, point_start.1, 0, None, None);
        let end_node = Node::new(point_end.0, point_end.1, 0, None, None);
        let mut closed_list = HashTable::new(2000, 0.75);
        let mut open_list = PriorityQueue::new(Some(start_node.clone()));

        while !open_list.is_empty() {
            println!("Open list: {:?}", open_list.get_queue()); // Debug print

            match open_list.poll() {
                Some(current_node) => {
                    println!("Processing node: {:?}", current_node); // Debug print

                    if self.is_equal(&current_node, &end_node) {
                        return Some(self.set_path(&current_node));
                    }

                    closed_list.insert(current_node.clone());

                    for neighbour in self.find_neighbours(&matrix, current_node.x, current_node.y) {
                        if !closed_list.search(&neighbour) {
                            if neighbour.parent.is_none() && !self.is_equal(&start_node, &neighbour)
                            {
                                self.calculate_node_data(&neighbour, &current_node, &end_node);
                                neighbour.parent = Some(Box::new(current_node.clone()));
                            }

                            if !open_list.contains(&neighbour) {
                                open_list.add(neighbour);
                            }
                        }
                    }
                }
                None => {
                    // Eğer açık liste boşsa ve hedef bulunamadıysa None döndür
                    return None;
                }
            }
        }

        None
    }
}
