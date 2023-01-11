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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test_max_depth() {
        assert_eq!(max_depth(tree!()), 0);
        assert_eq!(max_depth(tree!(1, 2, 3)), 2);
        assert_eq!(max_depth(tree!(1, None, 3)), 2);
        assert_eq!(max_depth(tree!(3, 9, 20, None, None, 15, 7)), 3);
    }
}
