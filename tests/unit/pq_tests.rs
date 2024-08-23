#[cfg(test)]
mod tests {
    use controlled_astar::priority_queue::{PriorityQueue, State};

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
