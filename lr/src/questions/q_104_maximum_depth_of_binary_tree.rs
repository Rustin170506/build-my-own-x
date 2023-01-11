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

pub fn max_depth_bfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let mut level = 0;
    let mut q: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
    q.push_front(root);

    while !q.is_empty() {
        for _ in 0..q.len() {
            let node = q.pop_front().unwrap();
            if node.as_ref().unwrap().borrow().left.is_some() {
                q.push_back(node.as_ref().unwrap().borrow_mut().left.take());
            }
            if node.as_ref().unwrap().borrow().right.is_some() {
                q.push_back(node.as_ref().unwrap().borrow_mut().right.take());
            }
        }
        level += 1
    }

    level
}

pub fn max_depth_dfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let mut depth = 1;
    let mut s = VecDeque::new();

    s.push_back((root, depth));

    while !s.is_empty() {
        let (n, d) = s.pop_back().unwrap();
        if n.is_some() {
            depth = i32::max(depth, d);
            s.push_back((n.as_ref().unwrap().borrow_mut().left.take(), d + 1));
            s.push_back((n.as_ref().unwrap().borrow_mut().right.take(), d + 1));
        }
    }

    depth
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test_max_depth() {
        assert_eq!(max_depth(tree!()), 0);
        assert_eq!(max_depth(tree!(1, 2, 3)), 2);
        assert_eq!(max_depth(tree!(1, None, 3)), 2);
        assert_eq!(max_depth(tree!(3, 9, 20, None, None, 15, 7)), 3);
    }

    #[test]
    fn test_max_depth_bfs() {
        assert_eq!(max_depth_bfs(tree!()), 0);
        assert_eq!(max_depth_bfs(tree!(1, 2, 3)), 2);
        assert_eq!(max_depth_bfs(tree!(1, None, 3)), 2);
        assert_eq!(max_depth_bfs(tree!(3, 9, 20, None, None, 15, 7)), 3);
    }

    #[test]
    fn test_max_depth_dfs() {
        assert_eq!(max_depth_dfs(tree!()), 0);
        assert_eq!(max_depth_dfs(tree!(1, 2, 3)), 2);
        assert_eq!(max_depth_dfs(tree!(1, None, 3)), 2);
        assert_eq!(max_depth_dfs(tree!(3, 9, 20, None, None, 15, 7)), 3);
    }
}
