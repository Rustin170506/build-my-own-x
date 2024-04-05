use std::cell::RefCell;
use std::rc::Rc;

pub struct Node<T> {
    value: T,
    parent: Option<Rc<RefCell<Node<T>>>>,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
}

pub struct BinaryTree<T> {
    root: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> BinaryTree<T>
where
    T: PartialOrd,
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
            } else {
                if node.borrow().right.is_none() {
                    new_node.borrow_mut().parent = Some(node.clone());
                    node.borrow_mut().right = Some(new_node.clone());
                    break;
                } else {
                    current = node.borrow().right.clone();
                }
            }
        }
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
}
