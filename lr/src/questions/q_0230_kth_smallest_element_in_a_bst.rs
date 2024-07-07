use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::utils::TreeNode;

pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    let mut n = 0;
    let mut stack = VecDeque::new();
    let mut current = root;

    while current.is_some() || !stack.is_empty() {
        while current.is_some() {
            stack.push_back(current.clone());
            let left = current.as_ref().unwrap().borrow().left.clone();
            current = left;
        }

        current = stack.pop_back().unwrap();
        n += 1;
        if n == k {
            return current.as_ref().unwrap().borrow().val;
        }
        let right = current.as_ref().unwrap().borrow().right.clone();
        current = right;
    }

    unreachable!()
}

#[test]
fn test_kth_smallest() {
    use crate::tree;
    assert_eq!(kth_smallest(tree!(3, 1, 4, None, 2), 1), 1);
    assert_eq!(kth_smallest(tree!(5, 3, 6, 2, 4, None, None, 1), 3), 3);
}
