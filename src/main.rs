use controlled_astar::astar::AStar;
use controlled_astar::node::Node;
fn main() {
    // 5x5'lik bir harita oluşturun
    let mut matrix = Vec::new();
    for y in 0..5 {
        let mut row = Vec::new();
        for x in 0..5 {
            row.push(Node::new(
                x, y, 0, // Blok değeri 0 (geçilebilir)
                None, None,
            ));
        }
        matrix.push(row);
    }

    // Haritada bazı engeller oluşturun
    matrix[2][1].set_block(1);
    matrix[2][2].set_block(1);
    matrix[2][3].set_block(1);

    // A* algoritması için bir nesne oluşturun
    let astar = AStar::new(matrix, 10, 14); // 10 ve 14, yatay-dikey ve çapraz maliyetler

    // Başlangıç ve bitiş noktalarını belirleyin
    let start = (0, 0);
    let end = (4, 4);

    // En kısa yolu bulun
    match astar.find_shortest_path(start, end) {
        Some(path) => {
            println!("En kısa yol bulundu:");
            for (x, y) in path {
                println!("({}, {})", x, y);
            }
        }
        None => {
            println!("Bir yol bulunamadı.");
        }
    }
}
