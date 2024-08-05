use crate::node::Node;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

#[derive(Clone, Eq, PartialEq)]
struct PriorityQueueNode {
    node: Node,
    f: usize,
}

impl Ord for PriorityQueueNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.cmp(&self.f) // Min-heap için ters sıralama
    }
}

impl PartialOrd for PriorityQueueNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct PriorityQueue {
    heap: BinaryHeap<PriorityQueueNode>,
    set: HashSet<Node>, // Elemanların var olup olmadığını kontrol etmek için HashSet
}

impl PriorityQueue {
    pub fn new() -> Self {
        PriorityQueue {
            heap: BinaryHeap::new(),
            set: HashSet::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn poll(&mut self) -> Option<Node> {
        if let Some(PriorityQueueNode { node, .. }) = self.heap.pop() {
            self.set.remove(&node);
            Some(node)
        } else {
            None
        }
    }

    pub fn add(&mut self, node: Node) {
        let f = node.get_f();
        let pq_node = PriorityQueueNode {
            node: node.clone(),
            f,
        };
        // Önceki f değerini içeren node varsa onu güncelle
        if self.set.contains(&node) {
            self.set.remove(&node);
            self.heap.retain(|n| n.node != node);
        }
        self.heap.push(pq_node);
        self.set.insert(node);
    }

    pub fn contains(&self, node: &Node) -> bool {
        self.set.contains(node)
    }

    pub fn get_queue(&self) -> Vec<Node> {
        self.heap
            .iter()
            .map(|pq_node| pq_node.node.clone())
            .collect()
    }
}
