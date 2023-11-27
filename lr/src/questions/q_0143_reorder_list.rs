use crate::utils::ListNode;

pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
    let mut curr = head.take();
    let mut length = 0;
    let mut temp = &curr;

    // Calculate the length of the list
    while let Some(node) = temp {
        temp = &node.next;
        length += 1;
    }

    if length < 2 {
        *head = curr.take();
        return;
    }

    // Split the list into two halves
    let mut first_half = &mut curr;
    for _ in 0..(length + 1) / 2 {
        first_half = &mut first_half.as_mut().unwrap().next;
    }

    // Reverse the second half of the list
    let mut second_half = None;
    let mut temp = first_half.take();
    while let Some(mut node) = temp {
        temp = node.next.take();
        node.next = second_half;
        second_half = Some(node);
    }

    // Merge the two halves
    let mut merged_head = None;
    let mut merged_tail = &mut merged_head;

    while let Some(mut left_node) = curr {
        curr = left_node.next.take();
        *merged_tail = Some(left_node);
        merged_tail = &mut merged_tail.as_mut().unwrap().next;

        if let Some(mut right_node) = second_half {
            second_half = right_node.next.take();
            *merged_tail = Some(right_node);
            merged_tail = &mut merged_tail.as_mut().unwrap().next;
        }
    }

    *head = merged_head.take();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list;

    #[test]
    fn test_reorder_list() {
        let mut list = list!(1, 2, 3, 4);
        reorder_list(&mut list);
        assert_eq!(list, list!(1, 4, 2, 3));
    }
}
