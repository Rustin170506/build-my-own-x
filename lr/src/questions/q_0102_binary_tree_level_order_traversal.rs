use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::utils::TreeNode;

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    if root.is_none() {
        return vec![];
    }
    let mut result = vec![];
    let mut q = VecDeque::new();
    q.push_back(root);

    while !q.is_empty() {
        let len = q.len();
        let mut level = vec![];
        for _ in 0..len {
            let node = q.pop_front().unwrap();
            if let Some(node) = node {
                level.push(node.as_ref().borrow().val);
                q.push_back(node.as_ref().borrow().left.clone());
                q.push_back(node.as_ref().borrow().right.clone());
            }
        }
        if !level.is_empty() {
            result.push(level);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::tree;

    use super::*;

    #[test]
    fn test_level_order() {
        assert_eq!(
            level_order(tree!(3, 9, 20, None, None, 15, 7)),
            vec![vec![3], vec![9, 20], vec![15, 7]]
        );
    }
}
