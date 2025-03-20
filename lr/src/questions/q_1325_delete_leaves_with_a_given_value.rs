use std::{cell::RefCell, rc::Rc};

use crate::utils::TreeNode;

pub fn remove_leaf_nodes(
    root: Option<Rc<RefCell<TreeNode>>>,
    target: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    root.as_ref()?;

    let node = root.unwrap();
    let left = node.borrow_mut().left.take();
    node.borrow_mut().left = remove_leaf_nodes(left, target);
    let right = node.borrow_mut().right.take();
    node.borrow_mut().right = remove_leaf_nodes(right, target);

    if node.borrow().left.is_none() && node.borrow().right.is_none() && node.borrow().val == target
    {
        return None;
    }
    Some(node)
}

#[test]
fn test_remove_leaf_nodes() {
    use crate::tree;

    let root = tree!(1, 2, 3, 2, null, 2, 4);
    let target = 2;
    let result = remove_leaf_nodes(root, target);
    let expected = tree!(1, null, 3, null, 4);
    assert_eq!(result, expected);
}
