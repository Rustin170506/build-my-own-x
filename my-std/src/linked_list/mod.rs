use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
/// A node in a singly linked list.
pub struct Node<T> {
    elem: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A singly linked list with a reference-counted `Node` type.
pub struct LinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
}

impl<T> LinkedList<T> {
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

    /// Return `true` if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Length of the list.
    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}
