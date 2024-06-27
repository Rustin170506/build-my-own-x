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
