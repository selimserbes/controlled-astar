use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};

// `Node` yapısını dışarıdan import ediyoruz
use crate::node::Node;

#[derive(Debug, Clone)]
struct PriorityQueueNode {
    priority: i32,
    node: Node,
}

impl PartialEq for PriorityQueueNode {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for PriorityQueueNode {}

impl PartialOrd for PriorityQueueNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.priority.cmp(&self.priority))
    }
}

impl Ord for PriorityQueueNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

pub struct PriorityQueue {
    heap: BinaryHeap<PriorityQueueNode>,
    entry_finder: HashMap<Node, PriorityQueueNode>,
    removed: PriorityQueueNode,
    counter: AtomicUsize,
}

impl PriorityQueue {
    pub fn new(first_item: Option<Node>) -> Self {
        let mut pq = PriorityQueue {
            heap: BinaryHeap::new(),
            entry_finder: HashMap::new(),
            removed: PriorityQueueNode {
                priority: i32::MAX,
                node: Node::default(), // Dummy node, assuming Node implements Default
            },
            counter: AtomicUsize::new(0),
        };
        if let Some(item) = first_item {
            pq.add(item);
        }
        pq
    }

    pub fn add(&mut self, item: Node) {
        if self.entry_finder.contains_key(&item) {
            self.remove_item(&item);
        }
        let count = self.counter.fetch_add(1, AtomicOrdering::SeqCst);
        let entry = PriorityQueueNode {
            priority: item.get_f(),
            node: item,
        };
        self.entry_finder.insert(entry.node.clone(), entry.clone());
        self.heap.push(entry);
    }

    fn remove_item(&mut self, item: &Node) {
        if let Some(entry) = self.entry_finder.remove(item) {
            let mut removed_entry = self.removed.clone();
            removed_entry.node = entry.node;
            self.entry_finder
                .insert(removed_entry.node.clone(), removed_entry);
        }
    }

    pub fn poll(&mut self) -> Option<Node> {
        while let Some(entry) = self.heap.pop() {
            if entry.node != self.removed.node {
                self.entry_finder.remove(&entry.node);
                return Some(entry.node);
            }
        }
        None
    }

    pub fn is_empty(&self) -> bool {
        self.entry_finder.is_empty()
    }

    pub fn contains(&self, item: &Node) -> bool {
        self.entry_finder.contains_key(item)
    }
}
