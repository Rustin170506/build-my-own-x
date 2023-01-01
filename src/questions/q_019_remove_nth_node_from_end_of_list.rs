use crate::utils::ListNode;

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode::new(0)));
    dummy.as_mut().unwrap().next = head;

    let mut postion = 0;
    let mut right = &dummy.as_ref().unwrap().next;
    let mut n = n;
    while n > 0 && right.is_some() {
        right = &right.as_ref().unwrap().next;
        n -= 1;
    }
    while right.is_some() {
        right = &right.as_ref().unwrap().next;
        postion += 1;
    }
    let mut left = &mut dummy;
    while postion > 0 {
        left = &mut left.as_mut().unwrap().next;
        postion -= 1;
    }
    left.as_mut().unwrap().next = left.as_mut().unwrap().next.as_mut().unwrap().next.take();
    dummy.unwrap().next
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list;

    #[test]
    fn test_find_median_sorted_arrays() {
        assert_eq!(remove_nth_from_end(list!(1), 1), list!());
        assert_eq!(remove_nth_from_end(list!(1, 2), 1), list!(1));
        assert_eq!(
            remove_nth_from_end(list!(1, 2, 3, 4, 5), 2),
            list!(1, 2, 3, 5)
        );
        assert_eq!(remove_nth_from_end(list!(1, 2, 3), 2), list!(1, 3));
    }
}
