use crate::utils::ListNode;

pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode { val, next: head }));

    let mut curr = &mut dummy;

    while curr.as_ref().unwrap().next.is_some() {
        if curr.as_deref().unwrap().next.as_ref().unwrap().val == val {
            curr.as_mut().unwrap().next = curr.as_mut().unwrap().next.as_mut().unwrap().next.take();
        } else {
            curr = &mut curr.as_mut().unwrap().next;
        }
    }

    dummy.as_mut().unwrap().next.take()
}

pub fn remove_elements2(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
    let mut prev = &mut dummy;
    let mut curr = prev.as_mut().unwrap().next.take();

    while let Some(mut node) = curr {
        if node.val == val {
            curr = node.next.take();
        } else {
            prev.as_mut().unwrap().next = Some(node);
            prev = &mut prev.as_mut().unwrap().next;
            curr = prev.as_mut().unwrap().next.take();
        }
    }

    dummy.unwrap().next
}

#[test]
fn test_remove_elements() {
    use crate::list;

    assert_eq!(
        remove_elements2(list!(1, 2, 6, 3, 4, 5, 6), 6),
        list!(1, 2, 3, 4, 5)
    );
    assert_eq!(remove_elements(list!(), 1), list!());
    assert_eq!(remove_elements(list!(7, 7, 7, 7), 7), list!());
}
