use std::{cell::RefCell, rc::Rc};

use crate::utils::TreeNode;

struct Codec {}

impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut vals: Vec<String> = Vec::new();
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<String>) {
            if root.is_none() {
                vals.push("N".to_owned());
                return;
            }

            vals.push(root.as_ref().unwrap().borrow().val.to_string());
            dfs(root.as_ref().unwrap().borrow().left.clone(), vals);
            dfs(root.as_ref().unwrap().borrow().right.clone(), vals);
        }
        dfs(root, &mut vals);
        vals.join(",")
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let vals: Vec<_> = data.split(',').collect();
        let mut i = 0;
        fn dfs(vals: &Vec<&str>, i: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
            let val = *vals.get(*i).unwrap();
            if val == "N" {
                *i = *i + 1;
                return None;
            }

            let mut node = TreeNode::new(val.parse::<i32>().unwrap());
            *i = *i + 1;
            node.left = dfs(vals, i);
            node.right = dfs(vals, i);

            Some(Rc::new(RefCell::new(node)))
        }

        dfs(&vals, &mut i)
    }
}

#[test]
fn test_297() {
    use crate::tree;
    let codec = Codec::new();
    let root = tree!(1, 2, 3, 4, 5, 6, 7);
    let serialized = codec.serialize(root.clone());
    let deserialized = codec.deserialize(serialized);
    assert_eq!(deserialized, root);
}
