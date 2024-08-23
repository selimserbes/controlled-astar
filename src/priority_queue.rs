use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// Represents the states used in the A* algorithm.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct State {
    pub cost: usize,
    pub position: (usize, usize),
}

impl Ord for State {
    /// Defines the ordering for `State`, ensuring that states with lower costs have higher priority.
    ///
    /// # Parameters
    /// - `other`: The other `State` to compare against.
    ///
    /// # Returns
    /// `Ordering` result of the comparison.
    fn cmp(&self, other: &Self) -> Ordering {
        // Higher priority for states with lower cost (min-heap behavior)
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    /// Provides partial ordering for `State`.
    ///
    /// # Parameters
    /// - `other`: The other `State` to compare against.
    ///
    /// # Returns
    /// `Ordering` result of the comparison.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Use the full ordering implementation from `Ord`
        Some(self.cmp(other))
    }
}

/// Priority queue used in the A* algorithm.
#[derive(Debug)]
pub struct PriorityQueue {
    heap: BinaryHeap<State>,
}

impl PriorityQueue {
    /// Creates a new, empty `PriorityQueue`.
    ///
    /// # Returns
    /// A new `PriorityQueue` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::PriorityQueue;
    ///
    /// // Create a new empty priority queue
    /// let mut open_set = PriorityQueue::new();
    ///
    /// // Check if the queue is empty
    /// assert!(open_set.is_empty());
    /// ```
    pub fn new() -> Self {
        PriorityQueue {
            heap: BinaryHeap::new(),
        }
    }

    /// Adds a new `State` to the queue.
    ///
    /// # Parameters
    /// - `state`: The state to be added to the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::{PriorityQueue, State};
    /// use std::collections::HashMap;
    ///
    /// // Create a new priority queue
    /// let mut open_set = PriorityQueue::new();
    ///
    /// // Create a state representing a node in the A* algorithm
    /// let state = State {
    ///     cost: 10, // f-score value calculated in AStar
    ///     position: (1, 2), // position of the node in the grid
    /// };
    ///
    /// // Add the state to the priority queue
    /// open_set.push(state);
    ///
    /// // The queue should no longer be empty
    /// assert!(!open_set.is_empty());
    /// ```
    pub fn push(&mut self, state: State) {
        // Add the state to the heap
        self.heap.push(state);
    }

    /// Removes and returns the `State` with the highest priority from the queue.
    ///
    /// # Returns
    /// The `State` with the highest priority (if available) or `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::{PriorityQueue, State};
    /// use std::collections::HashMap;
    ///
    /// // Create a new priority queue and add some states
    /// let mut open_set = PriorityQueue::new();
    /// open_set.push(State { cost: 10, position: (1, 2) });
    /// open_set.push(State { cost: 5, position: (2, 3) }); // Lower cost, higher priority
    ///
    /// // Remove and get the state with the highest priority
    /// if let Some(state) = open_set.pop() {
    ///     assert_eq!(state.cost, 5); // Lower cost should be popped first
    ///     assert_eq!(state.position, (2, 3));
    /// } else {
    ///     panic!("Expected a state, but got None");
    /// }
    /// ```
    pub fn pop(&mut self) -> Option<State> {
        // Remove and return the state with the highest priority
        self.heap.pop()
    }

    /// Checks if the queue is empty.
    ///
    /// # Returns
    /// A boolean indicating whether the queue is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::{PriorityQueue, State};
    ///
    /// // Create a new priority queue
    /// let mut open_set = PriorityQueue::new();
    ///
    /// // Initially, the queue should be empty
    /// assert!(open_set.is_empty());
    ///
    /// // Add a state to the queue
    /// open_set.push(State { cost: 10, position: (1, 2) });
    ///
    /// // Now the queue should not be empty
    /// assert!(!open_set.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        // Check if the heap is empty
        self.heap.is_empty()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_queue_push_and_pop() {
        let mut pq = PriorityQueue::new();

        let state1 = State {
            cost: 10,
            position: (1, 2),
        };
        let state2 = State {
            cost: 5,
            position: (2, 3),
        };
        let state3 = State {
            cost: 15,
            position: (0, 0),
        };

        pq.push(state1.clone());
        pq.push(state2.clone());
        pq.push(state3.clone());

        assert_eq!(pq.pop().unwrap(), state2);
        assert_eq!(pq.pop().unwrap(), state1);
        assert_eq!(pq.pop().unwrap(), state3);
        assert!(pq.is_empty());
    }

    #[test]
    fn test_priority_queue_push_order() {
        let mut pq = PriorityQueue::new();

        let state1 = State {
            cost: 10,
            position: (1, 2),
        };
        let state2 = State {
            cost: 5,
            position: (2, 3),
        };
        let state3 = State {
            cost: 15,
            position: (0, 0),
        };

        pq.push(state1.clone());
        pq.push(state2.clone());
        pq.push(state3.clone());

        assert_eq!(pq.pop().unwrap(), state2);
        assert_eq!(pq.pop().unwrap(), state1);
        assert_eq!(pq.pop().unwrap(), state3);
    }

    #[test]
    fn test_priority_queue_empty_after_pop() {
        let mut pq = PriorityQueue::new();
        pq.push(State {
            cost: 10,
            position: (1, 2),
        });

        assert!(!pq.is_empty());

        pq.pop();
        assert!(pq.is_empty());
    }

    #[test]
    fn test_priority_queue_multiple_pushes() {
        let mut pq = PriorityQueue::new();

        let state1 = State {
            cost: 20,
            position: (1, 2),
        };
        let state2 = State {
            cost: 15,
            position: (2, 3),
        };
        let state3 = State {
            cost: 10,
            position: (3, 4),
        };
        let state4 = State {
            cost: 5,
            position: (4, 5),
        };

        pq.push(state1.clone());
        pq.push(state2.clone());
        pq.push(state3.clone());
        pq.push(state4.clone());

        assert_eq!(pq.pop().unwrap(), state4);
        assert_eq!(pq.pop().unwrap(), state3);
        assert_eq!(pq.pop().unwrap(), state2);
        assert_eq!(pq.pop().unwrap(), state1);
        assert!(pq.is_empty());
    }

    #[test]
    fn test_priority_queue_no_pop_on_empty() {
        let mut pq = PriorityQueue::new();

        assert!(pq.pop().is_none());
    }
}
