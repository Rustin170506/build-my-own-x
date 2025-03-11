use crate::utils::TreeNode;
use std::{cell::RefCell, cmp::max, rc::Rc};

pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if node.is_none() {
            return (0, 0);
        }
        let left_pair = dfs(&node.as_ref().unwrap().borrow().left);
        let right_pair = dfs(&node.as_ref().unwrap().borrow().right);
        let with_root = node.as_ref().unwrap().borrow().val + left_pair.1 + right_pair.1;
        let without_root = max(left_pair.0, left_pair.1) + max(right_pair.0, right_pair.1);
        (with_root, without_root)
    }
    let result = dfs(&root);
    max(result.0, result.1)
}

#[test]
fn test_rob() {
    use crate::tree;
    let root = tree!(3, 2, 3, null, 3, null, 1);
    assert_eq!(rob(root), 7);
    let root = tree!(3, 4, 5, 1, 3, null, 1);
    assert_eq!(rob(root), 9);
}
