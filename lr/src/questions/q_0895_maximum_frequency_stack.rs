use std::collections::HashMap;

struct FreqStack {
    count: HashMap<i32, i32>,
    max_count: i32,
    stacks: HashMap<i32, Vec<i32>>,
}

impl FreqStack {
    fn new() -> Self {
        Self {
            count: HashMap::new(),
            max_count: 0,
            stacks: HashMap::new(),
        }
    }

    fn push(&mut self, val: i32) {
        let val_count = 1 + *self.count.entry(val).or_default();
        self.count.insert(val, val_count);
        if val_count > self.max_count {
            self.max_count = val_count;
        }
        self.stacks.entry(val_count).or_default().push(val);
    }

    fn pop(&mut self) -> i32 {
        let stack = self.stacks.get_mut(&self.max_count).unwrap();
        let val = stack.pop().unwrap();
        *self.count.get_mut(&val).unwrap() -= 1;
        if stack.is_empty() {
            self.max_count -= 1;
        }

        val
    }
}

#[test]
fn test_freq_stack() {
    let mut stack = FreqStack::new();
    stack.push(5);
    stack.push(7);
    stack.push(5);
    stack.push(7);
    stack.push(4);
    stack.push(5);
    assert_eq!(stack.pop(), 5);
    assert_eq!(stack.pop(), 7);
    assert_eq!(stack.pop(), 5);
    assert_eq!(stack.pop(), 4);
    assert_eq!(stack.pop(), 7);
    assert_eq!(stack.pop(), 5);
}
