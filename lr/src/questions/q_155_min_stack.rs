struct MinStack {
    elements: Vec<(i32, i32)>,
    min: i32,
}

impl MinStack {
    fn new() -> Self {
        Self {
            elements: Vec::new(),
            min: i32::MAX,
        }
    }

    fn push(&mut self, val: i32) {
        let last_min = self.min;
        if val < self.min {
            self.min = val;
        }
        self.elements.push((val, last_min));
    }

    fn pop(&mut self) {
        self.min = self.elements.pop().unwrap().1;
    }

    fn top(&self) -> i32 {
        self.elements.last().unwrap().0
    }

    fn get_min(&self) -> i32 {
        self.min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_stack() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        min_stack.push(-4);
        assert_eq!(min_stack.get_min(), -4);
        min_stack.pop();
        min_stack.pop();
        assert_eq!(min_stack.top(), 0);
        assert_eq!(min_stack.get_min(), -2);
    }
}
