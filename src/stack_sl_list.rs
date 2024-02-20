#[derive(Default, Debug)]
struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Default, Debug)]
pub struct List<T> {
    head: Option<Box<Node<T>>>,
}
impl<T> List<T> {
    pub fn push(&mut self, elem: T) {
        let old_head = self.head.take();
        self.head = Some(Box::new(Node {
            elem,
            next: old_head,
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

// -----------------------------------------------------------

#[cfg(test)]
mod test_stack {
    use crate::stack_sl_list::*;
    #[test]
    fn test_is_empty1_pos() {
        let mut l = List::default();
        l.push(1);
        l.pop();
        assert!(l.is_empty());
    }

    #[test]
    fn test_is_empty1_neg() {
        let mut l = List::default();
        l.push(1);
        assert!(!l.is_empty());
    }
}