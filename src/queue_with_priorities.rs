use std::cmp::Ordering;
use crate::queue_2_stacks::Queue;

#[derive(PartialEq)]
pub enum Priorities {
    Low,
    Normal,
    High,
}
impl PartialOrd for Priorities {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Low, Self::Normal) | (Self::Low, Self::High) => { Some(Ordering::Less) },
            (Self::Normal, Self::High) => { Some(Ordering::Less) },
            (Self::Normal, Self::Low) => { Some(Ordering::Greater) },
            (Self::High, Self::Low) => { Some(Ordering::Greater) },
            (Self::High, Self::Normal) => { Some(Ordering::Greater) },
            _ => { Some(Ordering::Equal) }
        }
    }
}

impl Default for Priorities {
    fn default() -> Self {
        Self::Normal
    }
}
struct SinglePQueue<T> {
    priority: Priorities,
    queue: Queue<T>
}
impl<T> SinglePQueue<T> where T: Default {
    fn new(p: Priorities) -> Self {
        Self {
            priority: p,
            queue: Queue::<T>::default(),
        }
    }
}

pub struct PQueue<T> {
    low: SinglePQueue<T>,
    normal: SinglePQueue<T>,
    high: SinglePQueue<T>,
}
impl<T> Default for PQueue<T> where T: Default {
    fn default() -> Self {
        Self {
            low: SinglePQueue::new(Priorities::Low),
            normal: SinglePQueue::new(Priorities::Normal),
            high: SinglePQueue::new(Priorities::High),
        }
    }
}
impl<T> PQueue<T> {
    pub fn enqueue(&mut self, p: Priorities, elem: T) {
        match p {
            Priorities::Low => { self.low.queue.enqueue(elem) }
            Priorities::Normal => { self.normal.queue.enqueue(elem) }
            Priorities::High => { self.high.queue.enqueue(elem) }
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if let Some(e) = self.high.queue.dequeue() {
            return Some(e);
        }
        if let Some(e) = self.normal.queue.dequeue() {
            return Some(e);
        }
        if let Some(e) = self.low.queue.dequeue() {
            return Some(e);
        }
        None
    }
}
