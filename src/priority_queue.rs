use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct State {
    pub cost: usize,
    pub position: (usize, usize),
}

impl Ord for State {
    // Defines the ordering for State by comparing the cost in descending order.
    // This ensures that the state with the lowest cost has the highest priority.
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    // Provides a partial comparison for State based on the defined ordering.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub struct PriorityQueue {
    heap: BinaryHeap<State>,
}

impl PriorityQueue {
    // Creates a new empty PriorityQueue.
    pub fn new() -> Self {
        PriorityQueue {
            heap: BinaryHeap::new(),
        }
    }

    // Pushes a new State into the priority queue.
    pub fn push(&mut self, state: State) {
        self.heap.push(state);
    }

    // Removes and returns the State with the highest priority (lowest cost).
    pub fn pop(&mut self) -> Option<State> {
        self.heap.pop()
    }

    // Checks if the priority queue is empty.
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
}
