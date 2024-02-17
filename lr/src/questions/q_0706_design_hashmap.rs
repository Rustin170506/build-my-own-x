//

#[derive(Clone)]
struct ListNode {
    key: i32,
    value: i32,
    next: Option<Box<ListNode>>, // next: Option<Rc<RefCell<ListNode>>>
}

struct MyHashMap {
    inner: Vec<Option<Box<ListNode>>>,
}

impl MyHashMap {
    fn new() -> Self {
        MyHashMap {
            inner: vec![
                Some(Box::new(ListNode {
                    key: i32::MAX,
                    value: i32::MAX,
                    next: None
                }));
                1000
            ],
        }
    }

    fn hash_key(&self, key: i32) -> usize {
        key as usize % self.inner.len()
    }

    fn put(&mut self, key: i32, value: i32) {
        let index = self.hash_key(key);

        let mut cur: &mut Option<Box<ListNode>> = &mut self.inner[index];
        let mut cur_next: &mut Option<Box<ListNode>> = &mut cur.as_mut().unwrap().next;

        while cur_next.is_some() {
            if cur_next.as_ref().unwrap().key == key {
                cur_next.as_mut().unwrap().value = value;
                return;
            }
            cur = cur_next;
            cur_next = &mut cur.as_mut().unwrap().next;
        }

        // insert
        let new_node = ListNode {
            key,
            value,
            next: None,
        };

        *cur_next = Some(Box::new(new_node));
    }

    fn get(&mut self, key: i32) -> i32 {
        let index = self.hash_key(key);
        let mut cur_next: &mut Option<Box<ListNode>> =
            &mut self.inner[index].as_mut().unwrap().next;
        while cur_next.is_some() {
            if cur_next.as_ref().unwrap().key == key {
                return cur_next.as_ref().unwrap().value;
            }
            cur_next = &mut cur_next.as_mut().unwrap().next;
        }

        -1
    }

    fn remove(&mut self, key: i32) {
        let index = self.hash_key(key);
        let cur: &mut Option<Box<ListNode>> = &mut self.inner[index];
        let mut cur_next: &mut Option<Box<ListNode>> = &mut cur.as_mut().unwrap().next;
        while cur_next.is_some() {
            if cur_next.as_ref().unwrap().key == key {
                // Do some stuff to delete the cur_next
                cur.as_mut().unwrap().next = cur_next.as_mut().unwrap().next.take();
                return;
            }
            cur_next = &mut cur_next.as_mut().unwrap().next;
        }
    }
}

#[test]
fn test_hashmap() {
    let mut obj = MyHashMap::new();
    obj.put(1, 2);
    assert_eq!(obj.get(1), 2);
    obj.put(1, 3);
    assert_eq!(obj.get(1), 3);
    obj.remove(1);
    assert_eq!(obj.get(1), -1);
    // remove a non-exist key
    obj.remove(1);
    assert_eq!(obj.get(1), -1);
}
