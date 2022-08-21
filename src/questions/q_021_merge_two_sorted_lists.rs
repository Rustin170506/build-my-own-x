use crate::utils::ListNode;

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1.clone(), list2.clone()) {
        (None, None) => None,
        (None, Some(l)) => Some(l),
        (Some(l), None) => Some(l),
        (Some(_), Some(_)) => {
            let mut head: Option<Box<ListNode>> = None;
            let mut next = &mut head;
            let (mut l1, mut l2) = (list1, list2);
            while l1.is_some() && l2.is_some() {
                if l1.as_ref().unwrap().val <= l2.as_ref().unwrap().val {
                    let l1_next = l1.as_mut().unwrap().next.take();
                    *next = l1.take();
                    l1 = l1_next;
                } else {
                    let l2_next = l2.as_mut().unwrap().next.take();
                    *next = l2.take();
                    l2 = l2_next;
                }
                next = &mut next.as_mut().unwrap().next;
            }
            if l1.is_some() {
                *next = l1;
            }
            if l2.is_some() {
                *next = l2;
            }
            head
        }
    }
}

#[cfg(test)]
mod tests {
    use super::merge_two_lists;
    use crate::list;
    use crate::utils::ListNode;

    #[test]
    fn test_merge_two_lists() {
        assert_eq!(merge_two_lists(list!(), list!(1, 2, 3)), list!(1, 2, 3));
        assert_eq!(merge_two_lists(list!(), list!()), list!());
        assert_eq!(
            merge_two_lists(list!(1, 2, 4), list!(1, 3, 4)),
            list!(1, 1, 2, 3, 4, 4)
        );
    }
}
