use std::{cell::RefCell, rc::Rc};

use crate::utils::TreeNode;

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if root.is_none() {
        return true;
    }
    fn symmetric(
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if left.is_none() && right.is_none() {
            return true;
        }
        if left.is_none() || right.is_none() {
            return false;
        }

        let left_val = left.as_ref().unwrap().borrow().val;
        let right_val = right.as_ref().unwrap().borrow().val;
        if left_val != right_val {
            return false;
        }
        symmetric(
            left.as_ref().unwrap().borrow().left.to_owned(),
            right.as_ref().unwrap().borrow().right.to_owned(),
        ) && symmetric(
            left.as_ref().unwrap().borrow().right.to_owned(),
            right.as_ref().unwrap().borrow().left.to_owned(),
        )
    }

    symmetric(
        root.as_ref().unwrap().borrow().left.to_owned(),
        root.as_ref().unwrap().borrow().right.to_owned(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test_is_symmetric() {
        assert!(!is_symmetric(tree!(1, 2, 3)));
        assert!(is_symmetric(tree!()));
        assert!(is_symmetric(tree!(1, 2, 2, 3, 4, 4, 3)));
        assert!(!is_symmetric(tree!(1, 2, 2, None, 3, None, 3)));
    }
}
