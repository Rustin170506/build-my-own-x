use std::{cell::RefCell, rc::Rc};

use crate::utils::TreeNode;

pub fn is_subtree(
    root: Option<Rc<RefCell<TreeNode>>>,
    sub_root: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    is_subtree_helper(&root, &sub_root)
}

fn is_subtree_helper(
    root: &Option<Rc<RefCell<TreeNode>>>,
    sub_root: &Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    if sub_root.is_none() {
        return true;
    }
    if root.is_none() {
        return false;
    }

    if is_same_tree(root, sub_root) {
        true
    } else {
        return is_subtree_helper(&root.as_ref().unwrap().borrow().left, sub_root)
            || is_subtree_helper(&root.as_ref().unwrap().borrow().right, sub_root);
    }
}

pub fn is_same_tree(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    if p.is_none() && q.is_none() {
        return true;
    }
    if p.is_none() || q.is_none() {
        return false;
    }

    if p.as_ref().unwrap().borrow().val == q.as_ref().unwrap().borrow().val {
        return is_same_tree(
            &p.as_ref().unwrap().borrow().left,
            &q.as_ref().unwrap().borrow().left,
        ) && is_same_tree(
            &p.as_ref().unwrap().borrow().right,
            &q.as_ref().unwrap().borrow().right,
        );
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test_is_subtree() {
        assert!(is_subtree(tree!(1, 2, 3), tree!(1, 2, 3)));
        assert!(is_subtree(tree!(3, 4, 5, 1, 2), tree!(4, 1, 2)));
        assert!(is_subtree(tree!(3, 4, 5, 1, 2), None));
        assert!(!is_subtree(
            tree!(3, 4, 5, 1, 2, None, None, None, None, 0),
            tree!(4, 1, 2)
        ));
    }
}
