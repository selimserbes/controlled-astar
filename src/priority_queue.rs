use crate::node::Node;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Clone, Eq, PartialEq)]
struct PriorityQueueNode {
    node: Node,
    f: usize,
}

impl Ord for PriorityQueueNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.cmp(&self.f)
    }
}

impl PartialOrd for PriorityQueueNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct PriorityQueue {
    heap: BinaryHeap<PriorityQueueNode>,
}

impl PriorityQueue {
    pub fn new(start_node: Node) -> Self {
        let mut heap = BinaryHeap::new();
        heap.push(PriorityQueueNode {
            node: start_node.clone(),
            f: start_node.get_f(),
        });
        PriorityQueue { heap }
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn poll(&mut self) -> Node {
        self.heap.pop().expect("Heap should not be empty").node
    }

    pub fn add(&mut self, node: Node) {
        self.heap.push(PriorityQueueNode {
            f: node.get_f(),
            node,
        });
    }

    pub fn get_queue(&self) -> Vec<&Node> {
        self.heap.iter().map(|pq_node| &pq_node.node).collect()
    }
}
