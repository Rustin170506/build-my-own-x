#![allow(clippy::assigning_clones)]

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// Node of a binary tree.
#[derive(Eq, PartialEq, Debug)]
pub struct Node<T> {
    value: T,
    parent: Option<Rc<RefCell<Node<T>>>>,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn has_left_child(&self) -> bool {
        self.left.is_some()
    }

    pub fn has_right_child(&self) -> bool {
        self.right.is_some()
    }

    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

/// Binary search tree.
/// It is a binary tree in which the left child of a node has a value less than the parent node,
/// and the right child of a node has a value greater than the parent node.
#[derive(Default)]
pub struct BinarySearchTree<T> {
    root: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> BinarySearchTree<T>
where
    T: PartialOrd + Clone,
{
    /// Create a new binary search tree.
    pub const fn new() -> Self {
        BinarySearchTree { root: None }
    }

    /// Insert a value into the binary search tree.
    pub fn insert(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            parent: None,
            left: None,
            right: None,
        }));

        if self.root.is_none() {
            self.root = Some(new_node);
            return;
        }

        let mut current = self.root.clone();
        while let Some(node) = current {
            if new_node.borrow().value < node.borrow().value {
                if node.borrow().left.is_none() {
                    new_node.borrow_mut().parent = Some(node.clone());
                    node.borrow_mut().left = Some(new_node.clone());
                    break;
                } else {
                    current = node.borrow().left.clone();
                }
            } else if node.borrow().right.is_none() {
                new_node.borrow_mut().parent = Some(node.clone());
                node.borrow_mut().right = Some(new_node.clone());
                break;
            } else {
                current = node.borrow().right.clone();
            }
        }
    }

    /// Search for a value in the binary search tree.
    pub fn search(&self, value: T) -> Option<T> {
        let node = self.find_node(value)?;
        let value = node.borrow().value.clone();
        Some(value)
    }

    /// Remove a value from the binary search tree.
    pub fn remove(&mut self, value: T) {
        let node = match self.find_node(value) {
            Some(node) => node,
            None => return,
        };

        let (parent, left, right) = {
            let node_borrow = node.borrow();
            (
                node_borrow.parent.clone(),
                node_borrow.left.clone(),
                node_borrow.right.clone(),
            )
        };

        let update_link = |parent: &Rc<RefCell<Node<T>>>,
                           node: &Rc<RefCell<Node<T>>>,
                           new_link: Option<Rc<RefCell<Node<T>>>>| {
            let mut parent_borrow = parent.borrow_mut();
            if parent_borrow.left.as_ref() == Some(node) {
                parent_borrow.left = new_link;
            } else {
                parent_borrow.right = new_link;
            }
        };

        match (left, right) {
            // Leaf node.
            (None, None) => {
                if let Some(parent) = parent {
                    update_link(&parent, &node, None);
                } else {
                    self.root = None;
                }
            }
            // Node with one child.
            (None, Some(child)) | (Some(child), None) => {
                if let Some(parent) = parent {
                    update_link(&parent, &node, Some(child));
                } else {
                    self.root = Some(child);
                }
            }
            // Node with two children.
            (Some(_), Some(right)) => {
                let min_node = Self::find_min_node(Some(right.clone())).unwrap();
                let min_value = min_node.borrow().value.clone();
                self.remove(min_value.clone());
                node.borrow_mut().value = min_value;
            }
        }
    }

    // Find a node with the given value.
    fn find_node(&self, value: T) -> Option<Rc<RefCell<Node<T>>>> {
        let mut current = self.root.clone();
        while let Some(node) = current {
            if value == node.borrow().value {
                return Some(node);
            } else if value < node.borrow().value {
                current = node.borrow().left.clone();
            } else {
                current = node.borrow().right.clone();
            }
        }
        None
    }

    /// Get the minimum value in the binary search tree.
    pub fn min(&self) -> Option<T> {
        self.root.as_ref()?;

        let min_node = Self::find_min_node(self.root.clone());
        min_node.map(|node| node.borrow().value.clone())
    }

    /// Get the maximum value in the binary search tree.
    pub fn max(&self) -> Option<T> {
        self.root.as_ref()?;

        let max_node = Self::find_max_node(self.root.clone());
        max_node.map(|node| node.borrow().value.clone())
    }

    // Find the node with the minimum value.
    fn find_min_node(node: Option<Rc<RefCell<Node<T>>>>) -> Option<Rc<RefCell<Node<T>>>> {
        node.as_ref()?;

        let mut current = node.clone();
        while let Some(node) = current {
            if node.borrow().left.is_none() {
                return Some(node);
            }
            current = node.borrow().left.clone();
        }
        None
    }

    // Find the node with the maximum value.
    fn find_max_node(node: Option<Rc<RefCell<Node<T>>>>) -> Option<Rc<RefCell<Node<T>>>> {
        node.as_ref()?;

        let mut current = node.clone();
        while let Some(node) = current {
            if node.borrow().right.is_none() {
                return Some(node);
            }
            current = node.borrow().right.clone();
        }
        None
    }

    /// Find the successor of a value in the binary search tree.
    pub fn find_successor(&self, value: T) -> Option<T> {
        let node = self.find_node(value)?;
        // If right subtree exists, the successor is the minimum of that subtree.
        if let Some(right) = node.borrow().right.clone() {
            return Some(
                Self::find_min_node(Some(right))
                    .unwrap()
                    .borrow()
                    .value
                    .clone(),
            );
        }
        // Otherwise, traverse up until we find a node that is a left child of its parent.
        let mut current = Some(node);
        while let Some(curr_node) = current {
            if let Some(parent) = curr_node.borrow().parent.clone() {
                if Rc::ptr_eq(parent.borrow().left.as_ref().unwrap(), &curr_node) {
                    return Some(parent.borrow().value.clone());
                }
                current = Some(parent);
            } else {
                break;
            }
        }
        None
    }
}

impl<T> BinarySearchTree<T> {
    /// Traverse the binary search tree in inorder.
    pub fn inorder<F>(&self, visit: F)
    where
        F: FnMut(&T),
    {
        let mut visit = visit;
        fn inorder_helper<T, F>(node: Option<Rc<RefCell<Node<T>>>>, visit: &mut F)
        where
            F: FnMut(&T),
        {
            if let Some(node) = node {
                let node_borrowed = node.borrow();
                inorder_helper(node_borrowed.left.clone(), visit);
                visit(&node_borrowed.value);
                inorder_helper(node_borrowed.right.clone(), visit);
            }
        }
        inorder_helper(self.root.clone(), &mut visit);
    }

    /// Traverse the binary search tree in inorder using an iterative approach.
    pub fn inorder_iterate<F>(&self, visit: F)
    where
        F: FnMut(&T),
    {
        let mut visit = visit;
        let mut stack = VecDeque::new();
        let mut current = self.root.clone();
        loop {
            while let Some(node) = current {
                let node_borrowed = node.borrow();
                stack.push_back(Some(node.clone()));
                current = node_borrowed.left.clone();
            }
            if stack.is_empty() {
                break;
            }
            let node = stack
                .pop_back()
                .expect("stack is not empty")
                .expect("node is not None");
            let node_borrowed = node.borrow();
            visit(&node_borrowed.value);
            current = node_borrowed.right.clone();
        }
    }

    /// Traverse the binary search tree in preorder.
    pub fn preorder<F>(&self, visit: F)
    where
        F: FnMut(&T),
    {
        let mut visit = visit;
        fn preorder_helper<T, F>(node: Option<Rc<RefCell<Node<T>>>>, visit: &mut F)
        where
            F: FnMut(&T),
        {
            if let Some(node) = node {
                let node_borrowed = node.borrow();
                visit(&node_borrowed.value);
                preorder_helper(node_borrowed.left.clone(), visit);
                preorder_helper(node_borrowed.right.clone(), visit);
            }
        }
        preorder_helper(self.root.clone(), &mut visit);
    }

    /// Traverse the binary search tree in preorder using an iterative approach.
    pub fn preorder_iterate<F>(&self, visit: F)
    where
        F: FnMut(&T),
    {
        let mut visit = visit;
        let mut stack = VecDeque::new();
        let mut current = self.root.clone();
        loop {
            while let Some(node) = current {
                let node_borrowed = node.borrow();
                visit(&node_borrowed.value);
                stack.push_back(node_borrowed.right.clone());
                current = node_borrowed.left.clone();
            }
            if stack.is_empty() {
                break;
            }
            current = stack.pop_back().expect("stack is not empty");
        }
    }

    /// Traverse the binary search tree in postorder.
    pub fn postorder<F>(&self, visit: F)
    where
        F: FnMut(&T),
    {
        let mut visit = visit;
        fn postorder_helper<T, F>(node: Option<Rc<RefCell<Node<T>>>>, visit: &mut F)
        where
            F: FnMut(&T),
        {
            if let Some(node) = node {
                let node_borrowed = node.borrow();
                postorder_helper(node_borrowed.right.clone(), visit);
                visit(&node_borrowed.value);
            }
        }
        postorder_helper(self.root.clone(), &mut visit);
    }

    /// Traverse the binary search tree in postorder using an iterative approach.
    pub fn postorder_iterate<F>(&self, visit: F)
    where
        F: FnMut(&T),
    {
        let mut visit = visit;
        let mut stack = VecDeque::new();
        if self.root.is_some() {
            stack.push_back(self.root.clone());
        }
        let mut current = self.root.clone();
        let push_children_to_stack = |stack: &mut VecDeque<Option<Rc<RefCell<Node<T>>>>>| {
            while let Some(top) = stack.back().cloned() {
                if let Some(top) = top {
                    if top.borrow().has_left_child() {
                        if top.borrow().has_right_child() {
                            stack.push_back(top.borrow().right.clone());
                        }
                        stack.push_back(top.borrow().left.clone());
                    } else {
                        stack.push_back(top.borrow().right.clone());
                    }
                } else {
                    break;
                }
            }
            stack.pop_back();
        };

        while !stack.is_empty() {
            if let Some(node) = current {
                let top = stack
                    .back()
                    .cloned()
                    .expect("stack is not empty")
                    .expect("node is not None");
                if let Some(parent) = &node.borrow().parent {
                    if !Rc::ptr_eq(&top, parent) {
                        push_children_to_stack(&mut stack);
                    }
                } else {
                    push_children_to_stack(&mut stack);
                }
                current = stack.pop_back().expect("stack is not empty");
                let node = current.clone().expect("node is not None");
                let node_borrowed = node.borrow();
                visit(&node_borrowed.value);
            }
        }
    }

    /// Traverse the binary search tree in level order.
    pub fn level_order<F>(&self, mut visit: F)
    where
        F: FnMut(&T),
    {
        let mut queue = VecDeque::new();
        if let Some(root) = &self.root {
            queue.push_back(root.clone());
        }

        while let Some(node) = queue.pop_front() {
            let node_borrowed = node.borrow();
            visit(&node_borrowed.value);
            if let Some(left) = &node_borrowed.left {
                queue.push_back(left.clone());
            }
            if let Some(right) = &node_borrowed.right {
                queue.push_back(right.clone());
            }
        }
    }
}

/// Iterator for the binary search tree.
pub struct BinaryTreeIterator<T> {
    current: Option<Rc<RefCell<Node<T>>>>,
    stack: Vec<Rc<RefCell<Node<T>>>>,
}

impl<T> Iterator for BinaryTreeIterator<T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = &self.current {
            self.stack.push(node.clone());
            let left = node.borrow().left.clone();
            self.current = left;
        }

        let node = self.stack.pop()?;
        self.current = node.borrow().right.clone();
        let value = node.borrow().value.clone();
        Some(value)
    }
}

impl<T> IntoIterator for BinarySearchTree<T>
where
    T: Clone,
{
    type Item = T;
    type IntoIter = BinaryTreeIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        BinaryTreeIterator {
            current: self.root,
            stack: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut tree = BinarySearchTree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(2);
        tree.insert(4);
        tree.insert(6);
        tree.insert(8);

        let res = tree.into_iter().collect::<Vec<_>>();
        assert_eq!(res, vec![2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn search() {
        let mut tree = BinarySearchTree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(2);
        tree.insert(4);
        tree.insert(6);
        tree.insert(8);

        assert_eq!(tree.search(5), Some(5));
        assert_eq!(tree.search(3), Some(3));
        assert_eq!(tree.search(7), Some(7));
        assert_eq!(tree.search(2), Some(2));
        assert_eq!(tree.search(4), Some(4));
        assert_eq!(tree.search(6), Some(6));
        assert_eq!(tree.search(8), Some(8));
        assert_eq!(tree.search(9), None);
    }

    #[test]
    fn min() {
        let mut tree = BinarySearchTree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(2);
        tree.insert(4);
        tree.insert(6);
        tree.insert(8);

        assert_eq!(tree.min(), Some(2));
    }

    #[test]
    fn max() {
        let mut tree = BinarySearchTree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(2);
        tree.insert(4);
        tree.insert(6);
        tree.insert(8);

        assert_eq!(tree.max(), Some(8));
    }

    #[test]
    fn remove() {
        let mut tree = BinarySearchTree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(2);
        tree.insert(4);
        tree.insert(6);
        tree.insert(8);

        tree.remove(5);
        let res = tree.into_iter().collect::<Vec<_>>();
        assert_eq!(res, vec![2, 3, 4, 6, 7, 8]);
    }

    #[test]
    fn inorder() {
        let mut tree = BinarySearchTree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(2);
        tree.insert(4);
        tree.insert(6);
        tree.insert(8);

        let mut res = Vec::new();
        tree.inorder(|value| res.push(*value));
        assert_eq!(res, vec![2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_inorder_iterate() {
        let mut tree = BinarySearchTree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(2);
        tree.insert(4);
        tree.insert(6);
        tree.insert(8);

        let mut res = Vec::new();
        tree.inorder_iterate(|value| res.push(*value));
        assert_eq!(res, vec![2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn preorder() {
        let mut tree = BinarySearchTree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(2);
        tree.insert(4);
        tree.insert(6);
        tree.insert(8);

        let mut res = Vec::new();
        tree.preorder(|value| res.push(*value));
        assert_eq!(res, vec![5, 3, 2, 4, 7, 6, 8]);
    }

    #[test]
    fn test_preorder_iterate() {
        let mut tree = BinarySearchTree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(2);
        tree.insert(4);
        tree.insert(6);
        tree.insert(8);

        let mut res = Vec::new();
        tree.preorder_iterate(|value| res.push(*value));
        assert_eq!(res, vec![5, 3, 2, 4, 7, 6, 8]);
    }

    #[test]
    fn postorder() {
        let mut tree = BinarySearchTree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(2);
        tree.insert(4);
        tree.insert(6);
        tree.insert(8);

        let mut res = Vec::new();
        tree.postorder(|value| res.push(*value));
        assert_eq!(res, vec![2, 4, 3, 6, 8, 7, 5]);
    }

    #[test]
    fn test_postorder_iterate() {
        let mut tree = BinarySearchTree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(2);
        tree.insert(4);
        tree.insert(6);
        tree.insert(8);

        let mut res = Vec::new();
        tree.postorder_iterate(|value| res.push(*value));
        assert_eq!(res, vec![2, 4, 3, 6, 8, 7, 5]);
    }

    #[test]
    fn level_order() {
        let mut tree = BinarySearchTree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(2);
        tree.insert(4);
        tree.insert(6);
        tree.insert(8);

        let mut res = Vec::new();
        tree.level_order(|value| res.push(*value));
        assert_eq!(res, vec![5, 3, 7, 2, 4, 6, 8]);
    }

    #[test]
    fn test_find_successor() {
        let mut tree = BinarySearchTree::new();
        tree.insert(20);
        tree.insert(8);
        tree.insert(22);
        tree.insert(4);
        tree.insert(12);
        tree.insert(10);
        tree.insert(14);

        assert_eq!(tree.find_successor(8), Some(10));
        assert_eq!(tree.find_successor(10), Some(12));
        assert_eq!(tree.find_successor(12), Some(14));
        assert_eq!(tree.find_successor(14), Some(20));
        assert_eq!(tree.find_successor(20), Some(22));
        assert_eq!(tree.find_successor(22), None);
    }
}
