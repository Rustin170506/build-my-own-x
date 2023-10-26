use crate::utils::ListNode;

pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    debug_assert!(head.is_some());
    let mut count = 0;

    let mut new_head = &head;
    while new_head.is_some() {
        debug_assert!(1 <= new_head.as_ref().unwrap().val);
        debug_assert!(new_head.as_ref().unwrap().val <= 100);
        count += 1;
        debug_assert!(count <= 100);
        new_head = &new_head.as_ref().unwrap().next;
    }

    let middle = count / 2;

    let mut head = head;
    let mut head = &mut head;
    for _ in 0..middle {
        head = &mut head.as_mut().unwrap().next;
    }

    head.take()
}

#[test]
fn test_middle_node() {
    use crate::list;

    let head = list![1, 2, 3, 4, 5];
    let middle = middle_node(head);
    assert_eq!(middle.unwrap().val, 3);

    let head = list![1, 2, 3, 4, 5, 6];
    let middle = middle_node(head);
    assert_eq!(middle.unwrap().val, 4);
}
