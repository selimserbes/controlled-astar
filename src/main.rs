use controlled_astar::astar::AStar;
use controlled_astar::node::Node;
use std::time::{Duration, Instant};

fn main() {
    // 4x4 matris harita
    // 0 -> geçilebilir alan
    // 1 -> blok (geçilemez alan)
    //let map = vec![
    //    vec![0, 0, 0, 0],
    //    vec![0, 1, 1, 0],
    //    vec![0, 1, 0, 0],
    //    vec![0, 0, 0, 0],
    //];

    let map = vec![
        vec![0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1],
        vec![0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1],
        vec![0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1],
        vec![0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 0],
        vec![0, 0, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0],
        vec![0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 0],
        vec![0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0],
    ];

    // Geçerli yönler (sağ, sol, yukarı, aşağı)
    let directions = vec![
        (1, 0),  // sağ
        (-1, 0), // sol
        (0, 1),  // aşağı
        (0, -1), // yukarı
    ];

    // Haritayı Node matrisine dönüştürme
    let matrix: Vec<Vec<Node>> = map
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, &block)| Node::new(x, y, block, directions.clone()))
                .collect()
        })
        .collect();

    // A* algoritması için başlangıç ve hedef noktalarını tanımlama
    let start = (1, 2); // sol üst köşe
    let end = (9, 1); // sağ alt köşe

    // A* algoritmasını başlat
    let mut astar = AStar::new(matrix, 10, 14); // düz yönler için maliyet: 10, çapraz yönler için maliyet: 14

    //let _start = Instant::now();
    astar.find_shortest_path(start, end);
    //let _duration = _start.elapsed();
    //println!("Geçen süre: {:?}", _duration);
}
