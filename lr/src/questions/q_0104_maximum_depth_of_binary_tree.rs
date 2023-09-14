use std::{cell::RefCell, rc::Rc};

use crate::utils::TreeNode;

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let left = 1 + max_depth(root.as_ref().unwrap().borrow().left.to_owned());
    let right = 1 + max_depth(root.as_ref().unwrap().borrow().right.to_owned());

    i32::max(left, right)
}

#[test]
fn test_max_depth() {
    use crate::tree;

    assert_eq!(3, max_depth(tree!(3, 9, 20, None, None, 15, 7)))
}
