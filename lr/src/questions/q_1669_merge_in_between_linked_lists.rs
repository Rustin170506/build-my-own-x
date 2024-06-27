use crate::utils::ListNode;

pub fn merge_in_between(
    list1: Option<Box<ListNode>>,
    a: i32,
    b: i32,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut list1 = list1;
    let mut list2 = list2;

    // Find node a-1 and node a.
    let mut current_node = &mut list1;
    let mut index = 0;
    let mut node_a = None;
    while let Some(node) = current_node {
        if index == a - 1 {
            // Find the node a.
            node_a = node.as_mut().next.take();
            // Move the list2 behind the node a-1.
            node.as_mut().next = list2.take();
        }
        index += 1;
        current_node = &mut node.next;
    }

    // Find the b position.
    let mut current_node = node_a;
    for _ in a..b {
        current_node = current_node.unwrap().next;
    }

    let mut list1_node = &mut list1;
    while let Some(node) = list1_node {
        list1_node = &mut node.next;
    }

    list1_node.replace(current_node.unwrap().next.take().unwrap());

    list1
}

pub fn merge_in_between_with_rebuild(
    list1: Option<Box<ListNode>>,
    a: i32,
    b: i32,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    // Dummy node to simplify operations
    let mut dummy = Box::new(ListNode {
        val: 0,
        next: list1,
    });
    let mut prev = &mut dummy;

    // Advance to the node before 'a'
    for _ in 0..a {
        prev = prev.next.as_mut().unwrap();
    }

    // 'first' will point to the node at position 'a'
    let mut first = prev.next.take();

    // Advance 'first' to the node after 'b', effectively skipping [a, b]
    for _ in a..=b {
        first = first.unwrap().next;
    }

    // Connect 'prev' to 'list2'
    prev.next = list2;

    // Advance 'prev' to the end of 'list2'
    while let Some(ref mut next) = prev.next {
        prev = next;
    }

    // Connect the end of 'list2' to 'first'
    prev.next = first;

    dummy.next
}

#[test]
fn test_merge_in_between() {
    use crate::list;

    let list1 = list![0, 1, 2, 3, 4, 5];
    let list2 = list![1000000, 1000001, 1000002];
    let a = 3;
    let b = 4;
    let expected = list![0, 1, 2, 1000000, 1000001, 1000002, 5];
    assert_eq!(merge_in_between(list1, a, b, list2), expected);

    let list1 = list![0, 1, 2, 3, 4, 5, 6];
    let list2 = list![1000000, 1000001, 1000002, 1000003, 1000004];
    let a = 2;
    let b = 5;
    let expected = list![0, 1, 1000000, 1000001, 1000002, 1000003, 1000004, 6];
    assert_eq!(merge_in_between(list1, a, b, list2), expected);
}

#[test]
fn test_merge_in_between_with_rebuild() {
    use crate::list;

    let list1 = list![0, 1, 2, 3, 4, 5];
    let list2 = list![1000000, 1000001, 1000002];
    let a = 3;
    let b = 4;
    let expected = list![0, 1, 2, 1000000, 1000001, 1000002, 5];
    assert_eq!(merge_in_between_with_rebuild(list1, a, b, list2), expected);

    let list1 = list![0, 1, 2, 3, 4, 5, 6];
    let list2 = list![1000000, 1000001, 1000002, 1000003, 1000004];
    let a = 2;
    let b = 5;
    let expected = list![0, 1, 1000000, 1000001, 1000002, 1000003, 1000004, 6];
    assert_eq!(merge_in_between_with_rebuild(list1, a, b, list2), expected);
}
