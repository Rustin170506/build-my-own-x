#[derive(Debug, Clone)]
struct ListNode {
    key: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(key: i32) -> Self {
        Self { key, next: None }
    }
}

struct MyHashSet {
    data: Vec<Option<Box<ListNode>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {
    fn new() -> Self {
        Self {
            data: vec![Some(Box::new(ListNode::new(-1))); 10_i32.pow(4) as usize],
        }
    }

    fn add(&mut self, key: i32) {
        let index = key as usize % self.data.len();
        let mut curr = &mut self.data[index];
        while curr.as_ref().unwrap().next.is_some() {
            if curr.as_ref().unwrap().next.as_ref().unwrap().key == key {
                return;
            }
            curr = &mut curr.as_mut().unwrap().next;
        }

        curr.as_mut().unwrap().next = Some(Box::new(ListNode::new(key)));
    }

    fn remove(&mut self, key: i32) {
        let index = key as usize % self.data.len();
        let mut curr = &mut self.data[index];
        while curr.as_ref().unwrap().next.is_some() {
            if curr.as_ref().unwrap().next.as_ref().unwrap().key == key {
                curr.as_mut().unwrap().next =
                    curr.as_mut().unwrap().next.as_mut().unwrap().next.take();
                return;
            }
            curr = &mut curr.as_mut().unwrap().next;
        }
    }

    fn contains(&self, key: i32) -> bool {
        let index = key as usize % self.data.len();
        let mut curr = &self.data[index];
        while curr.as_ref().unwrap().next.is_some() {
            if curr.as_ref().unwrap().next.as_ref().unwrap().key == key {
                return true;
            }
            curr = &curr.as_ref().unwrap().next;
        }
        false
    }
}

#[test]
fn test() {
    let mut hash_set = MyHashSet::new();
    hash_set.add(1);
    hash_set.add(2);
    assert!(hash_set.contains(1)); // returns true
    assert!(!hash_set.contains(3)); // returns false (not found)
    hash_set.add(2);
    assert!(hash_set.contains(2)); // returns true
    hash_set.remove(2);
    assert!(!hash_set.contains(2)); // returns false (already removed)
}
