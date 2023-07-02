use std::collections::{hash_map::Entry, HashMap};

struct DetectSquares {
    pts_count: HashMap<(i32, i32), i32>,
}
impl DetectSquares {
    fn new() -> Self {
        Self {
            pts_count: HashMap::new(),
        }
    }

    fn add(&mut self, point: Vec<i32>) {
        assert!(point.len() == 2);
        match self.pts_count.entry((point[0], point[1])) {
            Entry::Occupied(mut e) => {
                *e.get_mut() += 1;
            }
            Entry::Vacant(e) => {
                e.insert(1);
            }
        }
    }

    fn count(&self, point: Vec<i32>) -> i32 {
        let mut res = 0;
        let (x, y) = (point[0], point[1]);
        for (&(x1, y1), &c1) in self.pts_count.iter() {
            if x1 == x || y1 == y || (x1 - x).abs() != (y1 - y).abs() {
                continue;
            }
            if let Some(&c2) = self.pts_count.get(&(x1, y)) {
                if let Some(&c3) = self.pts_count.get(&(x, y1)) {
                    res += c1 * c2 * c3;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_squares() {
        let mut obj = DetectSquares::new();
        obj.add(vec![3, 10]);
        obj.add(vec![11, 2]);
        obj.add(vec![3, 2]);
        assert_eq!(obj.count(vec![11, 10]), 1);
        assert_eq!(obj.count(vec![14, 8]), 0);
        obj.add(vec![11, 2]);
        assert_eq!(obj.count(vec![11, 10]), 2);
    }
}
