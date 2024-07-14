use std::{cell::RefCell, rc::Rc};

use crate::utils::TreeNode;

pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut result = root.as_ref().unwrap().borrow().val;
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, result: &mut i32) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut left_max = dfs(root.as_ref().unwrap().borrow().left.clone(), result);
        let mut right_max = dfs(root.as_ref().unwrap().borrow().right.clone(), result);
        left_max = i32::max(left_max, 0);
        right_max = i32::max(right_max, 0);

        let root_value = root.as_ref().unwrap().borrow().val;
        *result = i32::max(*result, root_value + left_max + right_max);

        root_value + i32::max(left_max, right_max)
    }

    dfs(root, &mut result);

    result
}

#[test]
fn test_max_path_sum() {
    use crate::tree;
    assert_eq!(max_path_sum(tree!(1, 2, 3)), 6);
    assert_eq!(max_path_sum(tree!(-10, 9, 20, None, None, 15, 7)), 42);
}
