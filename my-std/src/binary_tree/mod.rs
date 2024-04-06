#![allow(clippy::assigning_clones)]

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Eq, PartialEq, Debug)]
pub struct Node<T> {
    value: T,
    parent: Option<Rc<RefCell<Node<T>>>>,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Default)]
pub struct BinaryTree<T> {
    root: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> BinaryTree<T>
where
    T: PartialOrd + Clone,
{
    pub fn new() -> Self {
        BinaryTree { root: None }
    }

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

    pub fn search(&self, value: T) -> Option<T> {
        let node = self.find_node(value)?;
        let value = node.borrow().value.clone();
        Some(value)
    }
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

    pub fn min(&self) -> Option<T> {
        self.root.as_ref()?;

        let min_node = Self::find_min_node(self.root.clone());
        min_node.map(|node| node.borrow().value.clone())
    }

    pub fn max(&self) -> Option<T> {
        self.root.as_ref()?;

        let max_node = Self::find_max_node(self.root.clone());
        max_node.map(|node| node.borrow().value.clone())
    }

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
}

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

impl<T> IntoIterator for BinaryTree<T>
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
        let mut tree = BinaryTree::new();
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
        let mut tree = BinaryTree::new();
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
        let mut tree = BinaryTree::new();
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
        let mut tree = BinaryTree::new();
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
        let mut tree = BinaryTree::new();
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
}
