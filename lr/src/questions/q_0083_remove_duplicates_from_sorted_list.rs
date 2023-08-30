use crate::utils::ListNode;

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut current = &mut head;
    while current.is_some() {
        let mut next = current.as_mut().unwrap().next.take();
        while next.is_some() && next.as_ref().unwrap().val == current.as_ref().unwrap().val {
            next = next.unwrap().next;
        }
        current.as_mut().unwrap().next = next.take();
        current = &mut current.as_mut().unwrap().next;
    }

    head
}
#[cfg(test)]
mod tests {
    use super::delete_duplicates;
    use crate::list;
    use crate::utils::ListNode;

    #[test]
    fn test_delete_duplicates() {
        assert_eq!(delete_duplicates(list!(1, 1, 1)), list!(1));
        assert_eq!(delete_duplicates(list!(1, 2, 3)), list!(1, 2, 3));
    }
}
