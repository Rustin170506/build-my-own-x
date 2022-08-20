// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[macro_export]
macro_rules! list {
    () => {
        None::<Box<ListNode>>
    };
    ($x:expr$(,)?)=> {
        Some(Box::new(ListNode::new($x)))
    };
    ($x:expr$(,$y:expr)+$(,)?) => {{
        let mut head = Some(Box::new(ListNode::new($x)));
        let mut next = &mut head;
        $(
            next = &mut next.as_mut().unwrap().next;
            *next = Some(Box::new(ListNode::new($y)));
        )*
        head
    }};
}
