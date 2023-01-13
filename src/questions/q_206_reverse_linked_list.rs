use crate::utils::ListNode;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut pre, mut cuur) = (None, head);

    while let Some(mut node) = cuur {
        cuur = node.next.take();
        node.next = pre;
        pre = Some(node);
    }

    pre
}

pub fn reverse_list_recursive(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn reverse(pre: Option<Box<ListNode>>, cur: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(mut node) = cur {
            let next = node.next.take();
            node.next = pre;
            reverse(Some(node), next)
        } else {
            pre
        }
    }

    reverse(None, head)
}

#[cfg(test)]
mod tests {
    use crate::list;

    use super::*;

    #[test]
    fn test_reverse_list() {
        assert_eq!(reverse_list(list!(1, 2, 3)), list!(3, 2, 1));
        assert_eq!(reverse_list_recursive(list!(1, 2, 3)), list!(3, 2, 1));
    }
}
