use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
/// A node in a singly linked list.
pub struct Node<T: Clone> {
    elem: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A singly linked list with a reference-counted `Node` type.
pub struct LinkedList<T: Clone> {
    head: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
}

impl<T: Clone> LinkedList<T> {
    /// Create a new, empty `LinkedList`.
    pub fn new() -> LinkedList<T> {
        LinkedList { head: None, len: 0 }
    }

    /// Add an element to the front of the list.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_std::linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push(1);
    /// list.push(2);
    /// assert_eq!(list.len(), 2);
    /// ```
    pub fn push(&mut self, elem: T) {
        let new_node = Rc::new(RefCell::new(Node {
            elem,
            next: self.head.clone(),
        }));

        self.head = Some(new_node);
        self.len += 1;
    }

    /// Remove an element from the front of the list.
    /// Returns `None` if the list is empty.
    ///
    /// # Examples
    /// ```
    /// use my_std::linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push(1);
    /// list.push(2);
    /// assert_eq!(list.pop(), Some(2));
    /// assert_eq!(list.pop(), Some(1));
    /// assert_eq!(list.pop(), None);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let next = self.head.as_mut().unwrap().borrow_mut().next.take();
        let res = self.head.take().unwrap();
        self.head = next;
        self.len -= 1;

        let res = res.borrow().elem.clone();
        Some(res)
    }

    /// Return `true` if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Length of the list.
    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T: Clone> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}