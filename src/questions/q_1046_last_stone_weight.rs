use std::collections::BinaryHeap;

pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
    let mut heap = BinaryHeap::from(stones);
    while heap.len() > 1 {
        let num1 = heap.pop().unwrap();
        let num2 = heap.pop().unwrap();

        let remain = (num1 - num2).abs();
        if remain == 0 {
            continue;
        }

        heap.push(remain);
    }

    if heap.is_empty() {
        0
    } else {
        *heap.peek().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1046() {
        assert_eq!(last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
    }
}
