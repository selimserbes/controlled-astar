// astar.rs
//
use crate::hash_table::HashTable;
use crate::node::Node;
use crate::priority_queue::{PriorityQueue, State};

fn manhattan_distance(start: (usize, usize), goal: (usize, usize)) -> usize {
    let (x1, y1) = start;
    let (x2, y2) = goal;

    (x1 as isize - x2 as isize).abs() as usize + (y1 as isize - y2 as isize).abs() as usize
}

pub fn astar_search(
    start: (usize, usize),
    goal: (usize, usize),
    nodes: &HashTable<(usize, usize), Node>,
) -> Option<Vec<(usize, usize)>> {
    let mut open_set = PriorityQueue::new();
    let start_state = State {
        cost: 0,
        position: start,
    };
    open_set.push(start_state);

    let mut came_from = HashTable::new();
    let mut g_score = HashTable::new();
    g_score.insert(start, 0);

    let mut f_score = HashTable::new();
    f_score.insert(start, manhattan_distance(start, goal));

    while let Some(current_state) = open_set.pop() {
        let current_position = current_state.position;

        if current_position == goal {
            let mut path = Vec::new();
            let mut current = current_position;

            while let Some(prev) = came_from.get(&current) {
                path.push(current);
                current = *prev; // Dereference the key here
            }

            path.push(start);
            path.reverse();
            return Some(path);
        }

        if let Some(current_node) = nodes.get(&current_position) {
            for &neighbor_position in current_node.neighbors.values() {
                if let Some(neighbor_pos) = neighbor_position {
                    if let Some(neighbor_node) = nodes.get(&neighbor_pos) {
                        if neighbor_node.is_blocked {
                            continue;
                        }

                        let tentative_g_score =
                            g_score.get(&current_position).unwrap_or(&usize::MAX) + 1;

                        if tentative_g_score < *g_score.get(&neighbor_pos).unwrap_or(&usize::MAX) {
                            came_from.insert(neighbor_pos, current_position);
                            g_score.insert(neighbor_pos, tentative_g_score);
                            let f_score_value =
                                tentative_g_score + manhattan_distance(neighbor_pos, goal);
                            f_score.insert(neighbor_pos, f_score_value);

                            let neighbor_state = State {
                                cost: f_score_value,
                                position: neighbor_pos,
                            };
                            open_set.push(neighbor_state);
                        }
                    }
                }
            }
        }
    }

    None
}
