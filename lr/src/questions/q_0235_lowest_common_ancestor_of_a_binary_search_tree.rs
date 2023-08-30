use std::{cell::RefCell, rc::Rc};

use crate::utils::TreeNode;

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let mut curr = root;
    let p = p.as_ref().unwrap().borrow().val;
    let q = q.as_ref().unwrap().borrow().val;
    while curr.is_some() {
        let value = curr.as_ref().unwrap().borrow().val;
        if q > value && p > value {
            let temp = curr.as_ref().unwrap().borrow().right.clone();
            curr = temp;
        } else if q < value && p < value {
            let temp = curr.as_ref().unwrap().borrow().left.clone();
            curr = temp;
        } else {
            return curr;
        }
    }
    unreachable!()
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
