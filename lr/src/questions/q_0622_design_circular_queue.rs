struct MyCircularQueue {
    queue: Vec<i32>,
    head: usize,
    tail: usize,
    size: usize,
    capacity: usize,
}

impl MyCircularQueue {
    fn new(k: i32) -> Self {
        MyCircularQueue {
            queue: vec![0; k as usize],
            head: 0,
            tail: 0,
            size: 0,
            capacity: k as usize,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.queue[self.tail] = value;
        self.tail = (self.tail + 1) % self.capacity;
        self.size += 1;
        true
    }

    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.head = (self.head + 1) % self.capacity;
        self.size -= 1;
        true
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.queue[self.head]
    }

    fn rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        let rear_index = (self.tail + self.capacity - 1) % self.capacity;
        self.queue[rear_index]
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }
}

#[test]
fn test_my_circular_queue() {
    let mut queue = MyCircularQueue::new(3);
    assert!(queue.en_queue(1));
    assert!(queue.en_queue(2));
    assert!(queue.en_queue(3));
    assert!(!queue.en_queue(4)); // Queue is full
    assert_eq!(queue.rear(), 3);
    assert!(queue.is_full());
    assert!(queue.de_queue());
    assert!(queue.en_queue(4));
    assert_eq!(queue.rear(), 4);
}
