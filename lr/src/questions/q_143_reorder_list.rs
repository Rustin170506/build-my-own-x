use crate::utils::ListNode;

pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
    let mut left = head.take();
    let mut n = 0;
    let mut p = &left;
    while let Some(node) = p {
        p = &node.next;
        n += 1;
    }
    if n < 2 {
        *head = left.take();
        return;
    }
    let mut p = &mut left;
    for _ in 0..(n + 1) / 2 {
        p = &mut p.as_mut().unwrap().next;
    }
    let mut right = None;
    let mut p = p.take();
    while let Some(mut node) = p {
        p = node.next.take();
        node.next = right;
        right = Some(node);
    }
    let mut new_head = None;
    let mut next = &mut new_head;
    while let Some(mut node) = left {
        left = node.next.take();
        *next = Some(node);
        next = &mut next.as_mut().unwrap().next;
        if let Some(mut node) = right {
            right = node.next.take();
            *next = Some(node);
            next = &mut next.as_mut().unwrap().next;
        }
    }
    *head = new_head.take();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list;

    #[test]
    fn test_reorder_list() {
        let mut list = list!(1, 2, 3, 4);
        reorder_list(&mut list);
        assert_eq!(list, list!(1, 4, 2, 3))
    }
}
