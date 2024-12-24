use std::{cell::RefCell, rc::Rc};

use crate::utils::TreeNode;

pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = vec![];

    traversal(&root, &mut result);

    result
}

fn traversal(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    if let Some(node) = root {
        result.push(node.as_ref().borrow().val);
        traversal(&node.as_ref().borrow().left, result);
        traversal(&node.as_ref().borrow().right, result);
    }
}

#[test]
fn test_preorder_traversal() {
    use crate::tree;

    assert_eq!(vec![1, 2, 3], preorder_traversal(tree!(1, 2, 3)));
    assert_eq!(vec![1, 2, 3], preorder_traversal(tree!(1, null, 2, 3)));
    assert_eq!(vec![1, 2, 3], preorder_traversal(tree!(1, 2, null, 3)));
}
