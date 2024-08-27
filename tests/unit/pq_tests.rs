#[cfg(test)]
mod tests {
    use controlled_astar::{PriorityQueue, State};

    #[test]
    fn test_priority_queue_push_and_pop() {
        // Create a new PriorityQueue instance
        let mut pq = PriorityQueue::new();

        // Create sample State instances with different costs
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

        // Push the states into the priority queue
        pq.push(state1.clone());
        pq.push(state2.clone());
        pq.push(state3.clone());

        // Verify that states are popped in the order of their priority (lowest cost first)
        assert_eq!(pq.pop().unwrap(), state2); // state2 should be first (lowest cost)
        assert_eq!(pq.pop().unwrap(), state1); // state1 should be next
        assert_eq!(pq.pop().unwrap(), state3); // state3 should be last (highest cost)
        assert!(pq.is_empty()); // Queue should be empty after all pops
    }

    #[test]
    fn test_priority_queue_push_order() {
        // Create a new PriorityQueue instance
        let mut pq = PriorityQueue::new();

        // Create sample State instances with different costs
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

        // Push the states into the priority queue
        pq.push(state1.clone());
        pq.push(state2.clone());
        pq.push(state3.clone());

        // Verify that states are popped in the correct order (lowest cost first)
        assert_eq!(pq.pop().unwrap(), state2); // state2 should be first
        assert_eq!(pq.pop().unwrap(), state1); // state1 should be next
        assert_eq!(pq.pop().unwrap(), state3); // state3 should be last
    }

    #[test]
    fn test_priority_queue_empty_after_pop() {
        // Create a new PriorityQueue instance
        let mut pq = PriorityQueue::new();

        // Push a single state into the queue
        pq.push(State {
            cost: 10,
            position: (1, 2),
        });

        assert!(!pq.is_empty()); // Queue should not be empty after pushing

        // Pop the state from the queue
        pq.pop();
        assert!(pq.is_empty()); // Queue should be empty after the pop
    }

    #[test]
    fn test_priority_queue_multiple_pushes() {
        // Create a new PriorityQueue instance
        let mut pq = PriorityQueue::new();

        // Create multiple State instances with different costs
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

        // Push multiple states into the queue
        pq.push(state1.clone());
        pq.push(state2.clone());
        pq.push(state3.clone());
        pq.push(state4.clone());

        // Verify that states are popped in the correct order (lowest cost first)
        assert_eq!(pq.pop().unwrap(), state4); // state4 should be first
        assert_eq!(pq.pop().unwrap(), state3); // state3 should be next
        assert_eq!(pq.pop().unwrap(), state2); // state2 should be next
        assert_eq!(pq.pop().unwrap(), state1); // state1 should be last
        assert!(pq.is_empty()); // Queue should be empty after all pops
    }

    #[test]
    fn test_priority_queue_no_pop_on_empty() {
        // Create a new PriorityQueue instance
        let mut pq = PriorityQueue::new();

        // Attempt to pop from an empty queue
        assert!(pq.pop().is_none()); // Should return None because the queue is empty
    }
}
