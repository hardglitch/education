use std::marker::PhantomData;
use std::ptr::NonNull;

type Link<T> = Option<NonNull<Node<T>>>;

#[derive(Default, Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

#[derive(Default, Debug)]
pub struct List<T> {
    head: Link<T>,
}
pub struct Iter<'a, T> {
    head: Link<T>,
    _dumb: PhantomData<&'a T>,
}
pub struct IterMut<'a, T> {
    head: Link<T>,
    _dumb: PhantomData<&'a mut T>,
}
pub struct IntoIter<T> {
    list: List<T>
}
impl<T> Extend<T> for List<T> {
    fn extend<I: IntoIterator<Item=T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item)
        }
    }
}
impl<T> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut list = Self::new();
        list.extend(iter);
        list
    }
}
impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
        }
    }
    
    pub fn push(&mut self, elem: T) {
        unsafe {
            let old_head = self.head.take();
            let new_head = NonNull::new_unchecked(Box::into_raw(Box::new(
                Node {
                    elem,
                    next: old_head,
                }
            )));

            self.head = Some(new_head);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            self.head.take().map(|node| {
                let boxed_node = Box::from_raw(node.as_ptr());
                self.head = boxed_node.next;
                boxed_node.elem
            })
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            head: self.head,
            _dumb: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            head: self.head,
            _dumb: PhantomData,
        }
    }
}
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(node) = cur_link {
            unsafe {
                let mut boxed_node = Box::from_raw(node.as_ptr());
                cur_link = boxed_node.next.take();
            }
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