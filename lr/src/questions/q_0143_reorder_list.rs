use std::{cell::RefCell, rc::Rc};

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

pub fn reorder_list1(head: &mut Option<Rc<RefCell<ListNode1>>>) {
    // Only one or two nodes in the list.
    if head.is_none() || head.as_ref().unwrap().borrow().next.is_none() {
        return;
    }

    // Split the list into two halves
    let mut slow = Rc::clone(head.as_ref().unwrap());
    let mut fast = Rc::clone(&slow);
    while fast.borrow().next.is_some()
        && fast.borrow().next.as_ref().unwrap().borrow().next.is_some()
    {
        let next_slow = Rc::clone(slow.borrow().next.as_ref().unwrap());
        slow = next_slow;

        let next_fast = Rc::clone(
            fast.borrow()
                .next
                .as_ref()
                .unwrap()
                .borrow()
                .next
                .as_ref()
                .unwrap(),
        );
        fast = next_fast;
    }

    // Split the list into two halves.
    let mut second_half = slow.borrow_mut().next.take();

    // Reverse the second half of the list
    let mut prev = None;
    let mut current = second_half;
    while let Some(node) = current {
        let next = node.borrow_mut().next.take();
        node.borrow_mut().next = prev.take();
        prev = Some(node);
        current = next;
    }
    second_half = prev;

    // Merge the two halves
    let mut first_half = head.take();
    let new_head = Rc::clone(first_half.as_ref().unwrap());

    while let Some(first_node) = first_half {
        first_half = first_node.borrow_mut().next.take();
        if let Some(second_node) = second_half {
            second_half = second_node.borrow_mut().next.take();
            first_node.borrow_mut().next = Some(Rc::clone(&second_node));
            if first_half.is_some() {
                second_node.borrow_mut().next = first_half.clone();
            } else {
                second_node.borrow_mut().next = None;
            }
        }
    }

    *head = Some(new_head)
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode1 {
    pub val: i32,
    pub next: Option<Rc<RefCell<ListNode1>>>,
}

#[test]
fn test_reorder_list1() {
    let node4 = Rc::new(RefCell::new(ListNode1 { val: 4, next: None }));
    let node3 = Rc::new(RefCell::new(ListNode1 {
        val: 3,
        next: Some(Rc::clone(&node4)),
    }));
    let node2 = Rc::new(RefCell::new(ListNode1 {
        val: 2,
        next: Some(Rc::clone(&node3)),
    }));
    let node1 = Rc::new(RefCell::new(ListNode1 {
        val: 1,
        next: Some(Rc::clone(&node2)),
    }));

    let mut head = Some(node1);
    reorder_list1(&mut head);

    // 1 4 2 3
    let expected_node4 = Rc::new(RefCell::new(ListNode1 { val: 3, next: None }));
    let expected_node3 = Rc::new(RefCell::new(ListNode1 {
        val: 2,
        next: Some(Rc::clone(&expected_node4)),
    }));
    let expected_node2 = Rc::new(RefCell::new(ListNode1 {
        val: 4,
        next: Some(Rc::clone(&expected_node3)),
    }));
    let expected_node1 = Rc::new(RefCell::new(ListNode1 {
        val: 1,
        next: Some(Rc::clone(&expected_node2)),
    }));
    let expected_head = Some(expected_node1);
    assert_eq!(head, expected_head);
}
