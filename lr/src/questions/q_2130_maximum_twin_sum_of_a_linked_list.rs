use crate::utils::ListNode;

pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
    // Find the middle position.
    let mut n = 0;
    let mut node = &head;
    while node.is_some() {
        n += 1;
        node = &node.as_ref().unwrap().next;
    }
    let middle = n / 2;

    // Find the middle node.
    let mut node = &head;
    for _ in 0..middle {
        node = &node.as_ref().unwrap().next;
    }
    // Reverse half of the list.
    let mut new_half_head = node.clone();
    let mut pre = None;
    while new_half_head.is_some() {
        let next = new_half_head.as_mut().unwrap().next.take();
        new_half_head.as_mut().unwrap().next = pre.take();
        pre = new_half_head;
        new_half_head = next;
    }

    // Get the max sum.
    let mut result = 0;
    let mut first_half = &head;
    let mut second_half = &pre;
    for _ in 0..middle {
        let first = first_half.as_ref().unwrap().val;
        let second = second_half.as_ref().unwrap().val;
        result = result.max(first + second);
        first_half = &first_half.as_ref().unwrap().next;
        second_half = &second_half.as_ref().unwrap().next;
    }

    result
}

#[test]
fn test_pair_sum() {
    use crate::list;

    assert_eq!(pair_sum(list![5, 4, 2, 1]), 6);
    assert_eq!(pair_sum(list![4, 2, 2, 3]), 7);
}
