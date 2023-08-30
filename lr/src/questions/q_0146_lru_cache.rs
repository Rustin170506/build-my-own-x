use std::{cell::RefCell, collections::HashMap, rc::Rc};

struct Node {
    key: i32,
    value: i32,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(key: i32, value: i32) -> Self {
        Node {
            key,
            value,
            prev: None,
            next: None,
        }
    }
}

struct LRUCache {
    cap: usize,
    cache: HashMap<i32, Rc<RefCell<Node>>>,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        let mut lru = LRUCache {
            cap: capacity as usize,
            cache: HashMap::new(),
            left: Some(Rc::new(RefCell::new(Node::new(0, 0)))),
            right: Some(Rc::new(RefCell::new(Node::new(0, 0)))),
        };

        lru.left.as_mut().unwrap().borrow_mut().next = lru.right.clone();
        lru.right.as_mut().unwrap().borrow_mut().prev = lru.left.clone();

        lru
    }

    fn remove(&mut self, node: Rc<RefCell<Node>>) {
        let prev = node.borrow().prev.clone();
        let next = node.borrow().next.clone();

        prev.as_ref().unwrap().borrow_mut().next = next.clone();
        next.as_ref().unwrap().borrow_mut().prev = prev;
    }

    fn insert(&mut self, node: Rc<RefCell<Node>>) {
        let prev = self.right.as_ref().unwrap().borrow().prev.clone();
        prev.as_ref().unwrap().borrow_mut().next = Some(node.clone());
        self.right.as_ref().unwrap().borrow_mut().prev = Some(node.clone());
        node.borrow_mut().prev = prev;
        node.borrow_mut().next = self.right.clone();
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.cache.get(&key) {
            let val = node.borrow().value;
            let node = node.clone();
            self.remove(node.clone());
            self.insert(node);
            return val;
        }
        -1
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.cache.get(&key) {
            node.borrow_mut().value = value;
            let node = node.clone();
            self.remove(node.clone());
            self.insert(node);
        } else {
            let node = Rc::new(RefCell::new(Node::new(key, value)));
            self.insert(node.clone());
            self.cache.insert(key, node);
        }

        if self.cache.len() > self.cap {
            let node = self.left.as_ref().unwrap().borrow().next.clone().unwrap();
            self.remove(node.clone());
            self.cache.remove(&node.borrow().key);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_146() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 3);
        assert_eq!(cache.get(2), -1);
        cache.put(4, 4);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }
}
