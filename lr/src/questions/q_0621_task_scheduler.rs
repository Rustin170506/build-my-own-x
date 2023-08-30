use std::collections::{BinaryHeap, HashMap, VecDeque};

pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
    let mut frequency: HashMap<char, i32> = HashMap::new();
    for task in tasks {
        // word is a &str
        *frequency.entry(task).or_insert(0) += 1; // word does not live long enough
    }

    let mut counts = frequency.into_values().collect::<Vec<i32>>();
    counts.sort();
    let mut heap = BinaryHeap::from(counts);

    let mut q = VecDeque::new();

    let mut time = 0;

    while !heap.is_empty() || !q.is_empty() {
        time += 1;

        if !heap.is_empty() {
            let cnt = heap.pop().unwrap();
            if cnt > 1 {
                q.push_back((cnt - 1, time + n));
            }
        } else {
            time = q.front().unwrap().1;
        }

        if !q.is_empty() && q.front().unwrap().1 == time {
            let item = q.pop_front().unwrap();
            heap.push(item.0);
        }
    }

    time
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_least_interval() {
        assert_eq!(least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2), 8);
        assert_eq!(least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 0), 6);
        assert_eq!(least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 50), 104);
    }
}
