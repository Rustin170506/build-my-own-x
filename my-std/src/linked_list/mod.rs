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

    /// Insert an element at the given index.
    /// Panics if the index is out of bounds.
    ///
    /// # Examples
    /// ```
    /// use my_std::linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push(1);
    /// list.push(2);
    /// list.insert(0, 3);
    /// assert_eq!(list.pop(), Some(2));
    /// assert_eq!(list.pop(), Some(3));
    /// assert_eq!(list.pop(), Some(1));
    /// ```
    pub fn insert(&mut self, index: usize, elem: T) {
        if index >= self.len() {
            panic!(
                "insertion index (is {index}) should be <= len (is {len})",
                len = self.len()
            );
        }

        let mut current = self.head.as_ref().unwrap().clone();
        for _ in 0..index {
            let next = current.borrow().next.as_ref().unwrap().clone();
            current = next;
        }

        let new_node = Rc::new(RefCell::new(Node {
            elem,
            next: current.borrow().next.clone(),
        }));

        current.borrow_mut().next = Some(new_node);
        self.len += 1;
    }

    /// Remove an element at the given index.
    /// Panics if the index is out of bounds.
    /// Returns the removed element.
    ///
    /// # Examples
    /// ```
    /// use my_std::linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push(1);
    /// list.push(2);
    /// assert_eq!(list.remove(1), 1);
    /// assert_eq!(list.remove(0), 2);
    /// ```
    pub fn remove(&mut self, index: usize) -> T {
        if index >= self.len() {
            panic!(
                "removal index (is {index}) should be < len (is {len})",
                index = index,
                len = self.len()
            );
        }

        let mut current = self.head.as_ref().unwrap().clone();
        let mut prev = None;
        for _ in 0..index {
            prev = Some(current.clone());
            let next = current.borrow().next.as_ref().unwrap().clone();
            current = next;
        }

        let res = current.borrow().elem.clone();
        let next = current.borrow_mut().next.take();
        if let Some(prev) = prev {
            prev.borrow_mut().next = next;
        } else {
            self.head = next;
        }
        self.len -= 1;

        res
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
