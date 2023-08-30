use std::{cell::RefCell, rc::Rc};

use crate::utils::TreeNode;

pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut result = 0;
    diameter_of_binary_tree_helper(root, &mut result);
    result
}

fn diameter_of_binary_tree_helper(root: Option<Rc<RefCell<TreeNode>>>, result: &mut i32) -> i32 {
    if root.is_none() {
        return -1;
    }

    let left =
        diameter_of_binary_tree_helper(root.as_ref().unwrap().borrow_mut().left.take(), result);
    let right =
        diameter_of_binary_tree_helper(root.as_ref().unwrap().borrow_mut().right.take(), result);

    *result = i32::max(*result, 2 + left + right);

    1 + i32::max(left, right)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test_diameter_of_binary_tree() {
        assert_eq!(diameter_of_binary_tree(tree!(1, 2)), 1);
        assert_eq!(diameter_of_binary_tree(tree!(1, 2, 3, 4, 5)), 3);
    }
}
