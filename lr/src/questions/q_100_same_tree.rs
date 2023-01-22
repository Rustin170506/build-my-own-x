use std::{cell::RefCell, rc::Rc};

use crate::utils::TreeNode;

pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if p.is_none() && q.is_none() {
        return true;
    }
    if p.is_none() || q.is_none() {
        return false;
    }

    if p.as_ref().unwrap().borrow().val == q.as_ref().unwrap().borrow().val {
        return is_same_tree(
            p.as_ref().unwrap().borrow().left.clone(),
            q.as_ref().unwrap().borrow().left.clone(),
        ) && is_same_tree(
            p.as_ref().unwrap().borrow().right.clone(),
            q.as_ref().unwrap().borrow().right.clone(),
        );
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test_inorder_is_same_tree() {
        assert!(is_same_tree(tree!(), tree!()));
        assert!(is_same_tree(tree!(1, 2, 3), tree!(1, 2, 3)));
        assert!(!is_same_tree(tree!(1, 2), tree!(1, None, 3)));
        assert!(!is_same_tree(tree!(1, 2, 1), tree!(1, 1, 2)));
        assert!(!is_same_tree(tree!(1, 1), tree!(1, None, 1)));
    }
}
