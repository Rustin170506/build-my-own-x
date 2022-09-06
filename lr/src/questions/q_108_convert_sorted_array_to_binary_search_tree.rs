use std::{cell::RefCell, rc::Rc};

use crate::utils::TreeNode;

pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.is_empty() {
        return None;
    }

    fn builder(nums: &Vec<i32>, left: i32, right: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if left > right {
            return None;
        }

        let mid = (left + right) / 2;
        let mut temp = TreeNode::new(nums[mid as usize]);

        temp.left = builder(nums, left, mid - 1);
        temp.right = builder(nums, mid + 1, right);

        Some(Rc::new(RefCell::new(temp)))
    }

    builder(&nums, 0, (nums.len() - 1) as i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test_sorted_array_to_bst() {
        assert_eq!(sorted_array_to_bst(vec![]), None);
        assert_eq!(
            sorted_array_to_bst(vec![-10, -3, 0, 5, 9]),
            tree!(0, -10, 5, None, -3, None, 9)
        );
    }
}
