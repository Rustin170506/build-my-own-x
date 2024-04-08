use std::{cell::RefCell, rc::Rc};

use crate::utils::TreeNode;

pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
    let mut root = root;
    root.as_ref()?;

    let val = root.as_ref().unwrap().borrow().val;
    if key > val {
        // Search in right side.
        let right = root.as_ref().unwrap().borrow().right.clone();
        root.as_mut().unwrap().borrow_mut().right = delete_node(right, key)
    } else if key < val {
        // Search in left side.
        let left = root.as_ref().unwrap().borrow().left.clone();
        root.as_mut().unwrap().borrow_mut().left = delete_node(left, key)
    } else if root.as_ref().unwrap().borrow().left.is_none() {
        // It only has one right child.
        return root.as_ref().unwrap().borrow().right.clone();
    } else if root.as_ref().unwrap().borrow().right.is_none() {
        // It only has one left child.
        return root.as_ref().unwrap().borrow().left.clone();
    } else {
        // It has two children.
        let right = root.as_ref().unwrap().borrow().right.clone();
        let min = find_min(right.clone());
        let val = min.as_ref().unwrap().borrow().val;
        root.as_mut().unwrap().borrow_mut().right = delete_node(right, val);
        root.as_mut().unwrap().borrow_mut().val = min.as_ref().unwrap().borrow().val;
    }

    root
}

fn find_min(current: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut cur = current;
    while cur.as_ref().unwrap().borrow().left.is_some() {
        let left = cur.as_ref().unwrap().borrow().left.clone();
        cur = left;
    }

    cur
}

#[test]
fn test_delete_node() {
    use crate::tree;

    let root = tree!(5, tree!(3, tree!(2), tree!(4)), tree!(6, None, tree!(7)));
    let key = 3;
    let res = tree!(5, tree!(4, tree!(2), None), tree!(6, None, tree!(7)));
    assert_eq!(delete_node(root, key), res);
}
