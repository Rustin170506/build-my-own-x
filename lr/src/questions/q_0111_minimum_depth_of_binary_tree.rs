use std::{cell::RefCell, rc::Rc};

use crate::utils::TreeNode;

pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let left = min_depth(root.as_ref().unwrap().borrow().left.to_owned());
    let right = min_depth(root.as_ref().unwrap().borrow().right.to_owned());

    if left == 0 || right == 0 {
        i32::max(left, right) + 1
    } else {
        i32::min(left, right) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test_min_depth() {
        assert_eq!(min_depth(tree!()), 0);
        assert_eq!(min_depth(tree!(3, 9, 20, None, None, 15, 7)), 2);
        assert_eq!(min_depth(tree!(2, None, 3, None, 4, None, 5, None, 6)), 5);
    }
}
