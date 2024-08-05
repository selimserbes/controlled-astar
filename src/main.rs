use controlled_astar::astar::AStar;
use controlled_astar::node::Node;

fn main() {
    // 5x5'lik bir matris oluşturuyoruz
    let matrix = vec![
        vec![
            Node::new(0, 0, Some(0), None, None),
            Node::new(1, 0, Some(0), None, None),
            Node::new(2, 0, Some(0), None, None),
            Node::new(3, 0, Some(0), None, None),
            Node::new(4, 0, Some(0), None, None),
        ],
        vec![
            Node::new(0, 1, Some(0), None, None),
            Node::new(1, 1, Some(0), None, None),
            Node::new(2, 1, Some(0), None, None),
            Node::new(3, 1, Some(0), None, None),
            Node::new(4, 1, Some(0), None, None),
        ],
        vec![
            Node::new(0, 2, Some(0), None, None),
            Node::new(1, 2, Some(0), None, None),
            Node::new(2, 2, Some(0), None, None),
            Node::new(3, 2, Some(0), None, None),
            Node::new(4, 2, Some(0), None, None),
        ],
        vec![
            Node::new(0, 3, Some(0), None, None),
            Node::new(1, 3, Some(0), None, None),
            Node::new(2, 3, Some(0), None, None),
            Node::new(3, 3, Some(0), None, None),
            Node::new(4, 3, Some(0), None, None),
        ],
        vec![
            Node::new(0, 4, Some(0), None, None),
            Node::new(1, 4, Some(0), None, None),
            Node::new(2, 4, Some(0), None, None),
            Node::new(3, 4, Some(0), None, None),
            Node::new(4, 4, Some(0), None, None),
        ],
    ];

    let hv_cost = 10;
    let diagonal_cost = 14;
    let mut astar = AStar::new(matrix, hv_cost, diagonal_cost);

    // Başlangıç ve bitiş noktalarını belirliyoruz
    let point_start = (0, 0);
    let point_end = (3, 2);

    // En kısa yolu bulmaya çalışıyoruz
    match astar.find_shortest_path(point_start, point_end) {
        Some(path) => {
            println!("En kısa yol bulundu:");
            for (x, y) in path {
                println!("({}, {})", x, y);
            }
        }
        None => println!("En kısa yol bulunamadı."),
    }
}
