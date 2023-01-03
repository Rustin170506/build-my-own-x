use crate::utils::ListNode;

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode::new(0)));
    let mut current = &mut dummy;
    let mut carry = 0;

    let mut l1 = &l1;
    let mut l2 = &l2;
    while l1.is_some() || l2.is_some() || carry != 0 {
        let v1 = if l1.is_some() {
            l1.as_ref().unwrap().val
        } else {
            0
        };
        let v2 = if l2.is_some() {
            l2.as_ref().unwrap().val
        } else {
            0
        };

        let mut val = v1 + v2 + carry;
        carry = val / 10;
        val %= 10;
        current.as_mut().unwrap().next = Some(Box::new(ListNode::new(val)));

        current = &mut current.as_mut().unwrap().next;
        l1 = if l1.is_some() && l1.as_ref().unwrap().next.is_some() {
            &l1.as_ref().unwrap().next
        } else {
            &None
        };
        l2 = if l2.is_some() && l2.as_ref().unwrap().next.is_some() {
            &l2.as_ref().unwrap().next
        } else {
            &None
        };
    }

    dummy.unwrap().next
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list;

    #[test]
    fn test_delete_duplicates() {
        assert_eq!(
            add_two_numbers(list!(1, 1, 1), list!(1, 1, 1)),
            list!(2, 2, 2)
        );
        assert_eq!(
            add_two_numbers(list!(2, 4, 3), list!(5, 6, 4)),
            list!(7, 0, 8)
        );
        assert_eq!(
            add_two_numbers(list!(9, 9, 9, 9, 9, 9, 9), list!(9, 9, 9, 9)),
            list!(8, 9, 9, 9, 0, 0, 0, 1)
        );
    }
}
