use std::{cell::RefCell, rc::Rc};

use crate::utils::TreeNode;

pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    fn helper(root: Option<Rc<RefCell<TreeNode>>>, sum: i32, target_sum: i32) -> bool {
        if root.is_none() {
            return false;
        }

        let sum = sum + root.as_ref().unwrap().borrow().val;

        if root.as_ref().unwrap().borrow().left.is_none()
            && root.as_ref().unwrap().borrow().right.is_none()
        {
            return sum == target_sum;
        }

        return helper(
            root.as_ref().unwrap().borrow().left.to_owned(),
            sum,
            target_sum,
        ) || helper(
            root.as_ref().unwrap().borrow().right.to_owned(),
            sum,
            target_sum,
        );
    }

    helper(root, 0, target_sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test_has_path_sum() {
        assert!(!has_path_sum(tree!(), 0));
        assert!(has_path_sum(
            tree!(5, 4, 8, 11, None, 13, 4, 7, 2, None, None, None, 1),
            22
        ));
        assert!(!has_path_sum(tree!(1, 2, 3), 5));
    }
}
