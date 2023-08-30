use std::{cell::RefCell, rc::Rc};

use crate::utils::TreeNode;

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    if root.is_none() {
        return vec![];
    }

    let mut stack = vec![];
    let mut result = vec![];

    let mut node = root;
    while node.is_some() || !stack.is_empty() {
        while node.is_some() {
            stack.push(node.clone());
            node = node.unwrap().borrow_mut().left.take();
        }
        if !stack.is_empty() {
            node = stack.pop().unwrap();
            result.push(node.as_ref().unwrap().borrow().val);
            node = node.unwrap().borrow_mut().right.take();
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test_inorder_traversal() {
        assert_eq!(inorder_traversal(tree!()), vec![]);
        assert_eq!(inorder_traversal(tree!(1)), vec![1]);
        assert_eq!(inorder_traversal(tree!(1, None, 2, 3)), vec![1, 3, 2]);
    }
}
