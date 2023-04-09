use std::{cell::RefCell, rc::Rc};

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(dead_code)]
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[macro_export]
macro_rules! tree {
    () => {
        None::<Rc<RefCell<TreeNode>>>
    };
    ($($x:expr),+$(,)?) => {{
        use std::collections::VecDeque;
        let mut nodes: VecDeque<_> = vec![$(stringify!($x)),*]
            .into_iter()
            .map(|v| {
                v.parse::<i32>()
                    .ok()
                    .map(|v| Rc::new(RefCell::new(TreeNode::new(v))))
            })
            .collect();
        let root = nodes.pop_front().unwrap();
        let mut queue = VecDeque::new();
        queue.push_back(root.as_ref().unwrap().clone());
        while !queue.is_empty() {
            let parent = queue.pop_front().unwrap();
            if let Some(Some(v)) = nodes.pop_front() {
                parent.borrow_mut().left = Some(v.clone());
                queue.push_back(v);
            }
            if let Some(Some(v)) = nodes.pop_front() {
                parent.borrow_mut().right = Some(v.clone());
                queue.push_back(v);
            }
        }
        root
    }};
}
