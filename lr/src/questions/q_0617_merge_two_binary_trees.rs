use std::{cell::RefCell, rc::Rc};

use crate::utils::TreeNode;

pub fn merge_trees(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    merge_trees_helper(root1, root2)
}

fn merge_trees_helper(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    match (root1, root2) {
        (None, None) => None,
        (Some(node), None) | (None, Some(node)) => Some(node),
        (Some(node1), Some(node2)) => {
            let val1 = node1.borrow().val;
            let val2 = node2.borrow().val;
            let new_node = Rc::new(RefCell::new(TreeNode::new(val1 + val2)));

            new_node.borrow_mut().left =
                merge_trees_helper(node1.borrow().left.clone(), node2.borrow().left.clone());
            new_node.borrow_mut().right =
                merge_trees_helper(node1.borrow().right.clone(), node2.borrow().right.clone());

            Some(new_node)
        }
    }
}

#[test]
fn test_merge_trees_helper() {
    use crate::tree;

    let tree1 = tree!(1, 3, 2, 5);
    let tree2 = tree!(2, 1, 3, null, 4, null, 7);
    let expected = tree!(3, 4, 5, 5, 4, null, 7);
    assert_eq!(merge_trees(tree1, tree2), expected);
}
