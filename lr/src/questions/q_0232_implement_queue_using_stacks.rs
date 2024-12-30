struct MyQueue {
    input: Vec<i32>,
    output: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        Self {
            input: Vec::new(),
            output: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.input.push(x);
    }

    fn pop(&mut self) -> i32 {
        self.move_input_to_output();
        self.output.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        self.move_input_to_output();
        *self.output.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.input.is_empty() && self.output.is_empty()
    }

    fn move_input_to_output(&mut self) {
        if self.output.is_empty() {
            while let Some(x) = self.input.pop() {
                self.output.push(x);
            }
        }
    }
}

#[test]
fn test_my_queue() {
    let mut obj = MyQueue::new();
    obj.push(1);
    obj.push(2);
    assert_eq!(obj.peek(), 1);
    assert_eq!(obj.pop(), 1);
    assert!(!obj.empty());
}
