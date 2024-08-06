mod astar;
mod hash_table;
mod node;
mod priority_queue;

use astar::AStar;
use node::Node;

fn main() {
    // Örnek bir grid matrisi oluşturuyoruz. 0 geçilebilir, 1 bloklu (geçilemez) alanları temsil ediyor.
    let grid = vec![
        vec![
            Node::new(0, 0, 0, None, None),
            Node::new(1, 0, 0, None, None),
            Node::new(2, 0, 0, None, None),
            Node::new(3, 0, 0, None, None),
        ],
        vec![
            Node::new(0, 1, 1, None, None),
            Node::new(1, 1, 1, None, None),
            Node::new(2, 1, 1, None, None),
            Node::new(3, 1, 0, None, None),
        ],
        vec![
            Node::new(0, 2, 1, None, None),
            Node::new(1, 2, 0, None, None),
            Node::new(2, 2, 0, None, None),
            Node::new(3, 2, 0, None, None),
        ],
        vec![
            Node::new(0, 3, 1, None, None),
            Node::new(1, 3, 0, None, None),
            Node::new(2, 3, 0, None, None),
            Node::new(3, 3, 0, None, None),
        ],
    ];

    // AStar nesnesini oluşturuyoruz.
    let mut astar = AStar::new(grid, 10, 14);

    // Başlangıç ve bitiş noktalarını belirliyoruz.
    let start = (0, 0);
    let end = (1, 2);

    // En kısa yolu bulmaya çalışıyoruz.
    match astar.find_shortest_path(start, end) {
        Some(path) => {
            println!("En kısa yol bulundu:");
            for step in path {
                println!("({},{})", step[0], step[1]);
            }
        }
        None => println!("Yol bulunamadı."),
    }
}
