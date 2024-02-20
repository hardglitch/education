use crate::stack_sl_list::List;

#[derive(Default)]
pub struct Queue<T> {
    ls: List<T>,
    rs: List<T>,
}

impl<T> Queue<T> {
    pub fn enqueue(&mut self, elem: T) {
        self.ls.push(elem);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        match self.rs.pop() {
            Some(elem) => { Some(elem) },
            None => {
                if !self.ls.is_empty() {
                    self.remove_from_ls_to_rs();
                    return self.dequeue()
                }
                None
            },
        }
    }

    fn remove_from_ls_to_rs(&mut self) {
        while let Some(a) = self.ls.pop() {
            self.rs.push(a);
        }
    }
}

// -----------------------------------------------------------

#[cfg(test)]
mod test_queue {
    use crate::queue_2_stacks::*;
    #[test]
    fn test_queue1_pos() {
        let mut q = Queue::default();
        q.enqueue(1);
        q.enqueue(2);
        q.enqueue(23);
        assert_eq!(1, q.dequeue().unwrap());
        assert_eq!(2, q.dequeue().unwrap());
        assert_eq!(23, q.dequeue().unwrap());
        assert!(q.dequeue().is_none());
    }
}