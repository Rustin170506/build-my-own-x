use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::utils::TreeNode;

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let left = 1 + max_depth(root.as_ref().unwrap().borrow().left.to_owned());
    let right = 1 + max_depth(root.as_ref().unwrap().borrow().right.to_owned());

    i32::max(left, right)
}

pub fn max_depth_queue(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let mut level = 0;
    let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
    queue.push_back(root);

    while !queue.is_empty() {
        let mut childen = VecDeque::new();
        for n in queue {
            if n.as_ref().unwrap().borrow().left.is_some() {
                childen.push_back(n.as_ref().unwrap().borrow().left.to_owned());
            }
            if n.as_ref().unwrap().borrow().right.is_some() {
                childen.push_back(n.as_ref().unwrap().borrow().right.to_owned());
            }
        }
        level += 1;
        queue = childen
    }

    level
}

#[test]
fn test_max_depth() {
    use crate::tree;

    assert_eq!(3, max_depth(tree!(3, 9, 20, None, None, 15, 7)));
    assert_eq!(3, max_depth_queue(tree!(3, 9, 20, None, None, 15, 7)));
}
