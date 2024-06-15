use std::collections::{hash_map::Entry, HashMap};

struct TimeMap {
    store: HashMap<String, Vec<(String, i32)>>,
}

impl TimeMap {
    fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        match self.store.entry(key) {
            Entry::Occupied(o) => o.into_mut().push((value, timestamp)),
            Entry::Vacant(v) => {
                v.insert(vec![(value, timestamp)]);
            }
        };
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        let mut res = "".to_string();
        let empty = vec![];
        let values = self.store.get(&key).unwrap_or(&empty);

        let (mut left, mut right) = (0_i32, (values.len() - 1) as i32);
        while left <= right {
            let mid = (left + right) / 2;

            if values[mid as usize].1 <= timestamp {
                res.clone_from(&values[mid as usize].0);
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_map() {
        let mut tm = TimeMap::new();
        tm.set("key".to_string(), "value".to_string(), 1);
        assert_eq!("value", tm.get("key".to_string(), 1));
        tm.set("key".to_string(), "value1".to_string(), 2);
        assert_eq!("value1", tm.get("key".to_string(), 3));
        tm.set("love".to_string(), "high".to_string(), 10);
        tm.set("love".to_string(), "low".to_string(), 20);
        assert_eq!("", tm.get("love".to_string(), 5));
    }
}
