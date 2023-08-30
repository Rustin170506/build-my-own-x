use crate::utils::ListNode;

pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut next = &mut head;
    loop {
        let mut start = next.take();
        let mut prev = &mut start;
        for _ in 0..k - 1 {
            if prev.is_none() {
                break;
            }
            prev = &mut prev.as_mut().unwrap().next;
        }
        if prev.is_none() {
            *next = start;
            break;
        }
        let mut prev = prev.as_mut().unwrap().next.take();
        while let Some(mut x) = start {
            start = x.next.take();
            x.next = prev;
            prev = Some(x);
        }
        *next = prev;
        for _ in 0..k {
            next = &mut next.as_mut().unwrap().next;
        }
    }
    head
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list;
    use crate::utils::ListNode;

    #[test]
    fn test_merge_two_lists() {
        assert_eq!(reverse_k_group(list!(1, 2, 3), 2), list!(2, 1, 3));
    }
}
