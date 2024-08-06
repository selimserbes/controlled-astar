use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::hash::Hash; // Hash trait'ini ekleyin

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PriorityQueueItem<T> {
    item: T,
    priority: i32,
}

impl<T> Ord for PriorityQueueItem<T>
where
    T: Eq,
{
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority) // Max-heap (en yüksek öncelik en üstte)
    }
}

impl<T> PartialOrd for PriorityQueueItem<T>
where
    T: Eq,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct PriorityQueue<T> {
    heap: BinaryHeap<PriorityQueueItem<T>>,
    entry_finder: HashMap<T, i32>, // Store the priority of the items
}

impl<T> PriorityQueue<T>
where
    T: Eq + Hash + Clone, // Hash trait'ini buraya ekleyin
{
    pub fn new() -> Self {
        PriorityQueue {
            heap: BinaryHeap::new(),
            entry_finder: HashMap::new(),
        }
    }

    pub fn add(&mut self, item: T, priority: i32) {
        let pq_item = PriorityQueueItem {
            item: item.clone(),
            priority,
        };
        self.heap.push(pq_item);
        self.entry_finder.insert(item, priority);
    }

    pub fn poll(&mut self) -> Option<T> {
        while let Some(popped_item) = self.heap.pop() {
            if let Some(&priority) = self.entry_finder.get(&popped_item.item) {
                if popped_item.priority == priority {
                    self.entry_finder.remove(&popped_item.item);
                    return Some(popped_item.item);
                }
            }
        }
        None
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn get_queue(&self) -> Vec<T> {
        self.heap.iter().map(|item| item.item.clone()).collect()
    }
}
