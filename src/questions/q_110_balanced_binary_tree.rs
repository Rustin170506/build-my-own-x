use std::{cell::RefCell, cmp::max, rc::Rc};

use crate::utils::TreeNode;

pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        return max(
            depth(root.as_ref().unwrap().borrow().left.to_owned()),
            depth(root.as_ref().unwrap().borrow().right.to_owned()),
        ) + 1;
    }

    if root.is_none() {
        return true;
    }

    let left = depth(root.as_ref().unwrap().borrow().left.to_owned());
    let right = depth(root.as_ref().unwrap().borrow().right.to_owned());

    return i32::abs(left - right) <= 1
        && is_balanced(root.as_ref().unwrap().borrow().left.to_owned())
        && is_balanced(root.as_ref().unwrap().borrow().right.to_owned());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test_is_balanced() {
        assert!(is_balanced(tree!()));
        assert!(is_balanced(tree!(3, 9, 20, None, None, 15, 7)));
        assert!(!is_balanced(tree!(1, 2, 2, 3, 3, None, None, 4, 4)));
    }
}
