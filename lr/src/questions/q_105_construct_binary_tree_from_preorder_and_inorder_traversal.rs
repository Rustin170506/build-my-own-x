use std::{cell::RefCell, rc::Rc};

use crate::utils::TreeNode;

pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn helper(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() || inorder.is_empty() {
            return None;
        }
        println!("preorder: {preorder:?}");
        println!("inorder: {inorder:?}");

        let first_value = *preorder.first().unwrap();
        let root = Some(Rc::new(RefCell::new(TreeNode::new(first_value))));
        let mid = inorder.iter().position(|&i| i == first_value).unwrap();

        root.as_ref().unwrap().borrow_mut().left = helper(&preorder[1..=mid], &inorder[..mid]);
        root.as_ref().unwrap().borrow_mut().right =
            helper(&preorder[mid + 1..], &inorder[mid + 1..]);

        root
    }

    helper(&preorder, &inorder)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test_build_tree() {
        assert_eq!(
            build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]),
            tree!(3, 9, 20, None, None, 15, 7)
        );
        // assert_eq!(build_tree(vec![1], vec![1]), tree!(1));
    }
}
