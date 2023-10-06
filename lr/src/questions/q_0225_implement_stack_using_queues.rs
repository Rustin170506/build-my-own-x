use std::collections::VecDeque;

struct MyStack {
    q: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    fn new() -> Self {
        Self { q: VecDeque::new() }
    }

    fn push(&mut self, x: i32) {
        self.q.push_back(x);
    }

    fn pop(&mut self) -> i32 {
        assert!(!self.q.is_empty());
        let len = self.q.len();
        for _ in 0..len - 1 {
            let n = self.q.pop_front().unwrap();
            self.q.push_back(n);
        }

        self.q.pop_front().unwrap()
    }

    fn top(&self) -> i32 {
        assert!(!self.q.is_empty());
        *self.q.back().unwrap()
    }

    fn empty(&self) -> bool {
        self.q.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0225() {
        let mut obj = MyStack::new();
        obj.push(1);
        obj.push(2);
        assert_eq!(obj.top(), 2);
        assert_eq!(obj.pop(), 2);
        assert!(!obj.empty());
    }
}
