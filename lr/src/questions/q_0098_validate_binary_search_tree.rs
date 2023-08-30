use std::{cell::RefCell, rc::Rc};

use crate::utils::TreeNode;

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn valid(root: Option<Rc<RefCell<TreeNode>>>, left: i64, right: i64) -> bool {
        if root.is_none() {
            return true;
        }

        let root_value = root.as_ref().unwrap().borrow().val as i64;
        if !(root_value > left && root_value < right) {
            return false;
        }

        return valid(
            root.as_ref().unwrap().borrow().left.clone(),
            left,
            root_value,
        ) && valid(
            root.as_ref().unwrap().borrow().right.clone(),
            root_value,
            right,
        );
    }

    valid(root, i64::MIN, i64::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test_inorder_is_valid_bst() {
        assert!(is_valid_bst(tree!(2147483647)));
        assert!(is_valid_bst(tree!(2, 1, 3)));
        assert!(!is_valid_bst(tree!(5, 1, 4, None, None, 3, 6)));
    }
}
