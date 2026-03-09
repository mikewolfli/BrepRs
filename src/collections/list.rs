use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct List<T> {
    head: RefCell<Option<Rc<Node<T>>>>,
    tail: RefCell<Option<Weak<Node<T>>>>,
    size: RefCell<usize>,
}

struct Node<T> {
    value: T,
    next: RefCell<Option<Rc<Node<T>>>>,
    prev: RefCell<Option<Weak<Node<T>>>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: RefCell::new(None),
            tail: RefCell::new(None),
            size: RefCell::new(0),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    pub fn size(&self) -> usize {
        *self.size.borrow()
    }

    pub fn clear(&self) {
        *self.head.borrow_mut() = None;
        *self.tail.borrow_mut() = None;
        *self.size.borrow_mut() = 0;
    }

    pub fn prepend(&self, value: T) {
        let new_node = Rc::new(Node {
            value,
            next: RefCell::new(self.head.borrow().clone()),
            prev: RefCell::new(None),
        });

        {
            let head_ref = self.head.borrow();
            if let Some(ref head) = *head_ref {
                *head.prev.borrow_mut() = Some(Rc::downgrade(&new_node));
            } else {
                *self.tail.borrow_mut() = Some(Rc::downgrade(&new_node));
            }
        }

        *self.head.borrow_mut() = Some(new_node);
        *self.size.borrow_mut() += 1;
    }

    pub fn append(&self, value: T) {
        let new_node = Rc::new(Node {
            value,
            next: RefCell::new(None),
            prev: RefCell::new(self.tail.borrow().clone()),
        });

        {
            let tail_ref = self.tail.borrow();
            if let Some(tail_weak) = tail_ref.as_ref() {
                if let Some(tail) = tail_weak.upgrade() {
                    *tail.next.borrow_mut() = Some(new_node.clone());
                }
            } else {
                *self.head.borrow_mut() = Some(new_node.clone());
            }
        }

        *self.tail.borrow_mut() = Some(Rc::downgrade(&new_node));
        *self.size.borrow_mut() += 1;
    }

    pub fn first(&self) -> Option<T>
    where
        T: Clone,
    {
        self.head.borrow().as_ref().map(|node| node.value.clone())
    }

    pub fn last(&self) -> Option<T>
    where
        T: Clone,
    {
        self.tail
            .borrow()
            .as_ref()
            .and_then(|weak| weak.upgrade())
            .map(|node| node.value.clone())
    }

    pub fn remove_first(&self) -> Option<T>
    where
        T: Clone,
    {
        let head = self.head.borrow_mut().take()?;
        let value = head.value.clone();

        {
            let next_ref = head.next.borrow();
            if let Some(ref next) = *next_ref {
                *next.prev.borrow_mut() = None;
                *self.head.borrow_mut() = Some(next.clone());
            } else {
                *self.tail.borrow_mut() = None;
            }
        }

        *self.size.borrow_mut() -= 1;
        Some(value)
    }

    pub fn remove_last(&self) -> Option<T>
    where
        T: Clone,
    {
        let tail_weak = self.tail.borrow_mut().take()?;
        let tail = tail_weak.upgrade()?;
        let value = tail.value.clone();

        {
            let prev_ref = tail.prev.borrow();
            if let Some(ref prev) = *prev_ref {
                if let Some(prev_strong) = prev.upgrade() {
                    *prev_strong.next.borrow_mut() = None;
                    *self.tail.borrow_mut() = Some(prev.clone());
                }
            } else {
                *self.head.borrow_mut() = None;
            }
        }

        *self.size.borrow_mut() -= 1;
        Some(value)
    }

    #[allow(dead_code)]
    fn insert_before(&self, node: &Rc<Node<T>>, value: T) {
        let new_node = Rc::new(Node {
            value,
            next: RefCell::new(Some(node.clone())),
            prev: RefCell::new(node.prev.borrow().clone()),
        });

        {
            let prev_ref = node.prev.borrow();
            if let Some(ref prev) = *prev_ref {
                if let Some(prev_strong) = prev.upgrade() {
                    *prev_strong.next.borrow_mut() = Some(new_node.clone());
                }
            } else {
                *self.head.borrow_mut() = Some(new_node.clone());
            }
        }

        *node.prev.borrow_mut() = Some(Rc::downgrade(&new_node));
        *self.size.borrow_mut() += 1;
    }

    #[allow(dead_code)]
    fn insert_after(&self, node: &Rc<Node<T>>, value: T) {
        let new_node = Rc::new(Node {
            value,
            next: RefCell::new(node.next.borrow().clone()),
            prev: RefCell::new(Some(Rc::downgrade(node))),
        });

        {
            let next_ref = node.next.borrow();
            if let Some(ref next) = *next_ref {
                *next.prev.borrow_mut() = Some(Rc::downgrade(&new_node));
            } else {
                *self.tail.borrow_mut() = Some(Rc::downgrade(&new_node));
            }
        }

        *node.next.borrow_mut() = Some(new_node);
        *self.size.borrow_mut() += 1;
    }

    pub fn extend(&self, other: &List<T>)
    where
        T: Clone,
    {
        let mut current = other.head.borrow().clone();
        while let Some(node) = current {
            self.append(node.value.clone());
            current = node.next.borrow().clone();
        }
    }

    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut result = Vec::with_capacity(self.size());
        let mut current = self.head.borrow().clone();
        while let Some(node) = current {
            result.push(node.value.clone());
            current = node.next.borrow().clone();
        }
        result
    }

    pub fn from_vec(vec: Vec<T>) -> Self {
        let list = Self::new();
        for value in vec {
            list.append(value);
        }
        list
    }

    pub fn iter(&self) -> ListIterator<T> {
        ListIterator {
            current: self.head.borrow().clone(),
        }
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clone for List<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        let new_list = Self::new();
        for value in self.iter() {
            new_list.append(value.clone());
        }
        new_list
    }
}

pub struct ListIterator<T> {
    current: Option<Rc<Node<T>>>,
}

impl<T> Iterator for ListIterator<T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.current.take()?;
        self.current = node.next.borrow().clone();
        Some(node.value.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_creation() {
        let list: List<i32> = List::new();
        assert!(list.is_empty());
        assert_eq!(list.size(), 0);
    }

    #[test]
    fn test_list_append() {
        let list = List::new();
        list.append(1);
        list.append(2);
        list.append(3);
        assert_eq!(list.size(), 3);
        assert_eq!(list.first(), Some(1));
        assert_eq!(list.last(), Some(3));
    }

    #[test]
    fn test_list_prepend() {
        let list = List::new();
        list.prepend(3);
        list.prepend(2);
        list.prepend(1);
        assert_eq!(list.size(), 3);
        assert_eq!(list.first(), Some(1));
        assert_eq!(list.last(), Some(3));
    }

    #[test]
    fn test_list_remove_first() {
        let list = List::new();
        list.append(1);
        list.append(2);
        list.append(3);
        assert_eq!(list.remove_first(), Some(1));
        assert_eq!(list.size(), 2);
        assert_eq!(list.first(), Some(2));
    }

    #[test]
    fn test_list_remove_last() {
        let list = List::new();
        list.append(1);
        list.append(2);
        list.append(3);
        assert_eq!(list.remove_last(), Some(3));
        assert_eq!(list.size(), 2);
        assert_eq!(list.last(), Some(2));
    }

    #[test]
    fn test_list_clear() {
        let list = List::new();
        list.append(1);
        list.append(2);
        list.append(3);
        list.clear();
        assert!(list.is_empty());
        assert_eq!(list.size(), 0);
    }

    #[test]
    fn test_list_iter() {
        let list = List::new();
        list.append(1);
        list.append(2);
        list.append(3);
        let values: Vec<i32> = list.iter().collect();
        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn test_list_to_vec() {
        let list = List::new();
        list.append(1);
        list.append(2);
        list.append(3);
        let vec = list.to_vec();
        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn test_list_from_vec() {
        let vec = vec![1, 2, 3];
        let list = List::from_vec(vec);
        assert_eq!(list.size(), 3);
        assert_eq!(list.to_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn test_list_extend() {
        let list1 = List::new();
        list1.append(1);
        list1.append(2);

        let list2 = List::new();
        list2.append(3);
        list2.append(4);

        list1.extend(&list2);
        assert_eq!(list1.size(), 4);
        assert_eq!(list1.to_vec(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_list_clone() {
        let list1 = List::new();
        list1.append(1);
        list1.append(2);
        list1.append(3);

        let list2 = list1.clone();
        assert_eq!(list2.size(), 3);
        assert_eq!(list2.to_vec(), vec![1, 2, 3]);
    }
}
