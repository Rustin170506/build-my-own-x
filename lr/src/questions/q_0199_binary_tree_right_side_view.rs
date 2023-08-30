use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::utils::TreeNode;

pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];
    let mut q = VecDeque::new();
    q.push_back(root);

    while !q.is_empty() {
        let mut right_side = None;
        let q_len = q.len();

        for _ in 0..q_len {
            let node = q.pop_front().unwrap();
            if node.is_some() {
                right_side = node.clone();
                q.push_back(node.as_ref().unwrap().borrow().left.clone());
                q.push_back(node.as_ref().unwrap().borrow().right.clone());
            }
        }

        if right_side.is_some() {
            res.push(right_side.as_ref().unwrap().borrow().val);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test_right_side_view() {
        assert_eq!(
            right_side_view(tree!(1, 2, 3, None, 5, None, 4)),
            vec![1, 3, 4]
        );
        assert_eq!(right_side_view(tree!()), vec![]);
    }
}
