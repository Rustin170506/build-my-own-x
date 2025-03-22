use std::collections::BinaryHeap;

pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
    let mut result = String::new();
    let mut max_heap = BinaryHeap::new();
    if a > 0 {
        max_heap.push((a, 'a'));
    }
    if b > 0 {
        max_heap.push((b, 'b'));
    }
    if c > 0 {
        max_heap.push((c, 'c'));
    }
    while !max_heap.is_empty() {
        let (mut count, char) = max_heap.pop().unwrap();
        if result.len() > 1
            && result.chars().last().unwrap() == char
            && result.chars().nth(result.len() - 2).unwrap() == char
        {
            if max_heap.is_empty() {
                break;
            }
            let (mut count2, char2) = max_heap.pop().unwrap();
            result.push(char2);
            count2 -= 1;
            if count2 != 0 {
                max_heap.push((count2, char2));
            }
        } else {
            result.push(char);
            count -= 1;
        }
        if count != 0 {
            max_heap.push((count, char));
        }
    }

    result
}

#[test]
fn test_longest_diverse_string() {
    assert_eq!(longest_diverse_string(1, 2, 7), "ccbccbccac");
    assert_eq!(longest_diverse_string(0, 0, 0), "");
}
