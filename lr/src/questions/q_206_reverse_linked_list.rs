use crate::utils::ListNode;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut prev, mut curr) = (None, head);
    while let Some(mut node) = curr {
        curr = node.next.take();
        node.next = prev;
        prev = Some(node);
    }

    prev
}

pub fn reverse_list_recursive(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn reverse(prev: Option<Box<ListNode>>, curr: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(mut node) = curr {
            let next = node.next.take();
            node.next = prev;
            reverse(Some(node), next)
        } else {
            prev
        }
    }

    reverse(None, head)
}

#[test]
fn test_reverse_list() {
    use crate::list;

    assert_eq!(reverse_list(list!()), list!());
    assert_eq!(reverse_list(list!(1, 2, 3, 4, 5)), list!(5, 4, 3, 2, 1));
    assert_eq!(reverse_list(list!(1)), list!(1));
}

#[test]
fn test_reverse_list_recursive() {
    use crate::list;

    assert_eq!(reverse_list_recursive(list!()), list!());
    assert_eq!(
        reverse_list_recursive(list!(1, 2, 3, 4, 5)),
        list!(5, 4, 3, 2, 1)
    );
    assert_eq!(reverse_list_recursive(list!(1)), list!(1));
}
