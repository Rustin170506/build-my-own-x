use crate::utils::ListNode;

pub fn reverse_between(
    head: Option<Box<ListNode>>,
    left: i32,
    right: i32,
) -> Option<Box<ListNode>> {
    // Handle empty list or no reversal needed
    if head.is_none() || left == right {
        return head;
    }

    // Create a dummy node to handle edge cases
    let mut dummy = Box::new(ListNode::new(0));
    dummy.next = head;
    let mut dummy = Some(dummy);

    // Find the node before the left position
    let mut pre = &mut dummy;
    for _ in 1..left {
        pre = &mut pre.as_mut().unwrap().next;
    }

    // Extract the sublist that needs to be reversed
    let mut current = pre.as_mut().unwrap().next.take();
    let mut reversed = None;

    // Reverse the sublist from left to right
    for _ in left..=right {
        if let Some(mut node) = current {
            current = node.next.take();
            node.next = reversed;
            reversed = Some(node);
        } else {
            break; // In case right exceeds list length
        }
    }

    // Find the tail of the reversed list
    let mut tail = &mut reversed;
    while tail.as_ref().unwrap().next.is_some() {
        tail = &mut tail.as_mut().unwrap().next;
    }

    // Connect the tail of the reversed list to the rest
    tail.as_mut().unwrap().next = current;

    // Connect the node before left to the new head of the reversed list
    pre.as_mut().unwrap().next = reversed;

    // Return the head, skipping the dummy node
    dummy.unwrap().next
}


#[test]
fn test_reverse_between() {
    use crate::list;
    assert_eq!(
        reverse_between(list![1, 2, 3, 4, 5], 2, 4),
        list![1, 4, 3, 2, 5]
    );
    assert_eq!(reverse_between(list![1], 1, 1), list![1]);
    assert_eq!(reverse_between(list![], 1, 1), list![]);
}
