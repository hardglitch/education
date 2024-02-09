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

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

    // pub fn into_iter(self) -> IntoIter<T> {
    //     IntoIter(self)
    // }
    //
    // pub fn iter(&self) -> Iter<'_, T> {
    //     Iter { next: self.head.as_deref() }
    // }
    //
    // pub fn iter_mut(&mut self) -> IterMut<'_, T> {
    //     IterMut { next: self.head.as_deref_mut() }
    // }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}
