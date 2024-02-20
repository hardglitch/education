use std::cmp::Ordering;
use crate::queue_2_stacks::Queue;
use crate::queue_with_priorities::Priorities;


#[derive(Default)]
struct Elem<T> {
    p: Priorities,
    value: T,
}
impl<T> PartialEq for Elem<T> {
    fn eq(&self, other: &Self) -> bool {
        self.p == other.p
    }
}
impl<T> PartialOrd for Elem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.p.partial_cmp(&other.p)
    }
}

#[derive(Default)]
pub struct PQueueSort<T> {
    q: Queue<Elem<T>>,
}

impl<T> PQueueSort<T> {
    pub fn enqueue(&mut self, p: Priorities, value: T) {
        let elem = Elem {p, value};
        self.q.enqueue(elem);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        
        //TODO!(Sort)

        if let Some(e) = self.q.dequeue() {
            return Some(e.value)
        }
        None
    }
}