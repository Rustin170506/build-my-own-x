use std::{cell::RefCell, mem::replace, rc::Rc};

use crate::utils::TreeNode;

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        return root;
    }

    let tmp1 = root.as_ref().unwrap().borrow_mut().left.take();
    let tmp2 = root.as_ref().unwrap().borrow_mut().right.take();
    let _ = replace(&mut root.as_ref().unwrap().borrow_mut().right, tmp1);
    let _ = replace(&mut root.as_ref().unwrap().borrow_mut().left, tmp2);

    invert_tree(root.as_ref().unwrap().borrow_mut().left.clone());
    invert_tree(root.as_ref().unwrap().borrow_mut().right.clone());

    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test_invert_tree() {
        assert_eq!(invert_tree(tree!()), tree!());
        assert_eq!(invert_tree(tree!(1)), tree!(1));
        assert_eq!(invert_tree(tree!(1, 2, 3)), tree!(1, 3, 2));
        assert_eq!(
            invert_tree(tree!(4, 2, 7, 1, 3, 6, 9)),
            tree!(4, 7, 2, 9, 6, 3, 1)
        );
    }
}
