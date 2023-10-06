use crate::utils::ListNode;

pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let mut head = head;
    let mut nums = vec![];

    while head.is_some() {
        nums.push(head.as_ref().unwrap().val);
        head = head.unwrap().next;
    }

    let (mut l, mut r) = (0, nums.len() - 1);
    while l < r {
        if nums[l] != nums[r] {
            return false;
        }
        l += 1;
        r -= 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list;

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome(list!(1, 2, 2, 1)));
        assert!(!is_palindrome(list!(1, 2)));
    }
}
