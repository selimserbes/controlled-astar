use controlled_astar::astar::astar_search;
use controlled_astar::hash_table::HashTable;
use controlled_astar::node::{Direction, Node};

fn matrix_to_nodes(matrix: &[Vec<i32>]) -> HashTable<(usize, usize), Node> {
    let mut hash_table = HashTable::new();
    let max_x = matrix.len();
    let max_y = matrix[0].len();

    for (x, row) in matrix.iter().enumerate() {
        for (y, &cell) in row.iter().enumerate() {
            let is_blocked = cell == 1;
            let node = Node::new(x, y, is_blocked, max_x - 1, max_y - 1);
            hash_table.insert((x, y), node);
        }
    }

    hash_table
}

fn main() {
    // 10x10'luk bir matris oluşturuyoruz. 0 = yol, 1 = engel.
    let matrix = vec![
        vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 1, 0, 1, 1, 1, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
        vec![0, 0, 0, 1, 1, 1, 0, 1, 0, 0],
        vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 1, 0, 0, 0, 1, 0],
        vec![0, 1, 0, 0, 0, 0, 0, 0, 1, 0],
        vec![0, 1, 1, 1, 0, 1, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 1, 1, 0, 0, 1, 1, 0],
    ];

    // Matrisi Node'lara dönüştürüyoruz
    let mut nodes = matrix_to_nodes(&matrix);

    // Örnek bir konum
    let position = (0, 0);

    if let Some(node) = nodes.get_mut(&position) {
        //node.remove_neighbor(Direction::South); // Güney yönünü kaldır
        node.remove_neighbor(Direction::West); // Batı yönünü kaldır
        node.set_neighbor(Direction::SouthEast, Some((position.0 + 1, position.1 + 1)));
        // Kuzeydoğu yönü
    }

    if let Some(node) = nodes.get_mut(&(0, 0)) {
        println!("Node ({}, {}): {:?}", 0, 0, node.get_directions());
    }

    // Başlangıç ve bitiş noktalarını tanımlıyoruz
    let start = (0, 0);
    let goal = (9, 9);

    // A* algoritmasını kullanarak yol bulma
    if let Some(path) = astar_search(start, goal, &nodes) {
        println!("Yol bulundu: {:?}", path);
    } else {
        println!("Yol bulunamadı.");
    }
}
