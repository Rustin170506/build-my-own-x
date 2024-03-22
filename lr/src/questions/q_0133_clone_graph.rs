//

use std::{cell::RefCell, rc::Rc};

pub struct Node {
    pub val: i32,
    pub neighbors: Vec<Rc<RefCell<Node>>>,
}

pub fn clone_graph(node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
    node.as_ref()?;

    let mut map = std::collections::HashMap::new();
    fn dfs(
        node: Rc<RefCell<Node>>,
        map: &mut std::collections::HashMap<*const Node, Rc<RefCell<Node>>>,
    ) -> Rc<RefCell<Node>> {
        match map.get(&(node.as_ptr() as *const Node)) {
            Some(n) => n.clone(),
            None => {
                let new_node = Rc::new(RefCell::new(Node {
                    val: node.borrow().val,
                    neighbors: vec![],
                }));
                map.insert(node.as_ptr() as *const Node, new_node.clone());
                for neighbor in node.borrow().neighbors.iter() {
                    new_node
                        .borrow_mut()
                        .neighbors
                        .push(dfs(neighbor.clone(), map));
                }
                new_node
            }
        }
    }

    Some(dfs(node.unwrap(), &mut map))
}

#[test]
fn test_clone_graph() {
    use std::collections::HashSet;

    let node1 = Rc::new(RefCell::new(Node {
        val: 1,
        neighbors: vec![],
    }));
    let node2 = Rc::new(RefCell::new(Node {
        val: 2,
        neighbors: vec![],
    }));
    let node3 = Rc::new(RefCell::new(Node {
        val: 3,
        neighbors: vec![],
    }));
    let node4 = Rc::new(RefCell::new(Node {
        val: 4,
        neighbors: vec![],
    }));
    node1.borrow_mut().neighbors = vec![node2.clone(), node4.clone()];
    node2.borrow_mut().neighbors = vec![node1.clone(), node3.clone()];
    node3.borrow_mut().neighbors = vec![node2.clone(), node4.clone()];
    node4.borrow_mut().neighbors = vec![node1.clone(), node3.clone()];

    fn dfs(
        node: Option<Rc<RefCell<Node>>>,
        clone: Option<Rc<RefCell<Node>>>,
        visited: &mut HashSet<i32>,
    ) {
        match (node, clone) {
            (Some(n), Some(c)) => {
                assert_ne!(n.as_ptr(), c.as_ptr());
                if visited.contains(&n.borrow().val) {
                    return;
                }
                visited.insert(n.borrow().val);
                assert_eq!(n.borrow().val, c.borrow().val);
                assert_eq!(n.borrow().neighbors.len(), c.borrow().neighbors.len());
                for (neighbor, clone_neighbor) in
                    n.borrow().neighbors.iter().zip(c.borrow().neighbors.iter())
                {
                    dfs(
                        Some(neighbor.clone()),
                        Some(clone_neighbor.clone()),
                        visited,
                    );
                }
            }
            _ => unreachable!("node and clone should be both Some"),
        }
    }

    let cloned = clone_graph(Some(node1.clone()));
    let mut visited = HashSet::new();
    dfs(Some(node1.clone()), cloned.clone(), &mut visited);
}
