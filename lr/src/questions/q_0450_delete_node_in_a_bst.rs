use std::{cell::RefCell, rc::Rc};

use crate::utils::TreeNode;

pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
    let mut root = root;
    if root.is_none() {
        return None;
    }

    let val = root.as_ref().unwrap().borrow().val;
    if key > val {
        let right = root.as_ref().unwrap().borrow().right.clone();
        root.as_mut().unwrap().borrow_mut().right = delete_node(right, key)
    } else if key < val {
        let left = root.as_ref().unwrap().borrow().left.clone();
        root.as_mut().unwrap().borrow_mut().left = delete_node(left, key)
    } else {
        if root.as_ref().unwrap().borrow().left.is_none() {
            return root.as_ref().unwrap().borrow().right.clone();
        } else if root.as_ref().unwrap().borrow().right.is_none() {
            return root.as_ref().unwrap().borrow().left.clone();
        } else {
            let mut cur = root.as_ref().unwrap().borrow().right.clone();
            while cur.as_ref().unwrap().borrow().left.is_some() {
                let left = cur.as_ref().unwrap().borrow().left.clone();
                cur = left;
            }
            root.as_mut().unwrap().borrow_mut().val = cur.as_ref().unwrap().borrow().val;
            let val = root.as_ref().unwrap().borrow().val;
            let right = root.as_ref().unwrap().borrow().right.clone();
            root.as_mut().unwrap().borrow_mut().right = delete_node(right, val);
        }
    }

    root
}

#[test]
fn test_delete_node() {
    use crate::tree;

    let root = tree!(5, tree!(3, tree!(2), tree!(4)), tree!(6, None, tree!(7)));
    let key = 3;
    let res = tree!(5, tree!(4, tree!(2), None), tree!(6, None, tree!(7)));
    assert_eq!(delete_node(root, key), res);
}
