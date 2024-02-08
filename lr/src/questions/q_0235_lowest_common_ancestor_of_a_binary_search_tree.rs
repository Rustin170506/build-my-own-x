use std::{cell::RefCell, rc::Rc};

use crate::utils::TreeNode;

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let mut current = root;
    let p = p.as_ref().unwrap().borrow().val;
    let q = q.as_ref().unwrap().borrow().val;

    while current.is_some() {
        let current_val = current.as_ref().unwrap().borrow().val;
        if current_val > p && current_val > q {
            let left = current.as_mut().unwrap().borrow_mut().left.take();
            current = left;
        } else if current_val < p && current_val < q {
            let right = current.as_mut().unwrap().borrow_mut().right.take();
            current = right;
        } else {
            return current;
        }
    }

    unreachable!("should return in while loop");
}

#[cfg(test)]
mod tests {
    use crate::tree;

    use super::*;

    #[test]
    fn test_235() {
        assert_eq!(
            lowest_common_ancestor(
                tree!(6, 2, 8, 0, 4, 7, 9, None, None, 3, 5),
                tree!(2),
                tree!(8)
            ),
            tree!(6, 2, 8, 0, 4, 7, 9, None, None, 3, 5)
        );
    }
}
