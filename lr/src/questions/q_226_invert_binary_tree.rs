use crate::utils::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        return root;
    }

    let temp1 = root.as_ref().unwrap().borrow_mut().left.take();
    let temp2 = root.as_ref().unwrap().borrow_mut().right.take();
    let _ = std::mem::replace(&mut root.as_ref().unwrap().borrow_mut().right, temp1);
    let _ = std::mem::replace(&mut root.as_ref().unwrap().borrow_mut().left, temp2);

    invert_tree(root.as_ref().unwrap().borrow_mut().left.clone());
    invert_tree(root.as_ref().unwrap().borrow_mut().right.clone());

    root
}

#[test]
fn test_invert_tree() {
    use crate::tree;

    assert_eq!(
        invert_tree(tree!(4, 2, 7, 1, 3, 6, 9)),
        tree!(4, 7, 2, 9, 6, 3, 1)
    );
    assert_eq!(invert_tree(tree!()), tree!());
}
