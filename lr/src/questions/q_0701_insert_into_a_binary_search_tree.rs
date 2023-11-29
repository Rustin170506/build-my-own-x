use std::{cell::RefCell, rc::Rc};

use crate::utils::TreeNode;

pub fn insert_into_bst(
    root: Option<Rc<RefCell<TreeNode>>>,
    val: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    let mut root = root;
    if root.is_none() {
        return Some(Rc::new(RefCell::new(TreeNode::new(val))));
    }

    if val < root.as_ref().unwrap().borrow().val {
        let result = insert_into_bst(root.as_mut().unwrap().borrow_mut().left.take(), val);
        root.as_mut().unwrap().borrow_mut().left = result;
    } else {
        let result = insert_into_bst(root.as_mut().unwrap().borrow_mut().right.take(), val);
        root.as_mut().unwrap().borrow_mut().right = result;
    }

    root
}

#[test]
fn test_solution() {
    use crate::tree;
    let root = tree![4, 2, 7, 1, 3];
    let val = 5;
    let result = tree![4, 2, 7, 1, 3, 5];
    assert_eq!(insert_into_bst(root, val), result);

    let root = tree![40, 20, 60, 10, 30, 50, 70];
    let val = 25;
    let result = tree![40, 20, 60, 10, 30, 50, 70, null, null, 25];
    assert_eq!(insert_into_bst(root, val), result);

    let root = tree![4, 2, 7, 1, 3, null, null, null, null, null, null];
    let val = 5;
    let result = tree![4, 2, 7, 1, 3, 5];
    assert_eq!(insert_into_bst(root, val), result);
}
