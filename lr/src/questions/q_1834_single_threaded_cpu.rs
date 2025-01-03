use std::{cmp::Reverse, collections::BinaryHeap};

pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
    let mut tasks = tasks;
    let mut result = vec![];
    for (i, task) in tasks.iter_mut().enumerate() {
        task.push(i as i32);
    }
    tasks.sort_by(|a, b| a[0].cmp(&b[0]));

    let mut heap = BinaryHeap::new();
    let (mut index, mut time) = (0, tasks[0][0]);

    while !heap.is_empty() || index < tasks.len() {
        while index < tasks.len() && time >= tasks[index][0] {
            heap.push(Reverse((tasks[index][1], tasks[index][2])));
            index += 1;
        }

        if heap.is_empty() {
            time = tasks[index][0];
        } else {
            let Reverse((processing_time, i)) = heap.pop().unwrap();
            time += processing_time;
            result.push(i);
        }
    }

    result
}

#[test]
fn test_get_order() {
    assert_eq!(
        get_order(vec![vec![1, 2], vec![2, 4], vec![3, 2], vec![4, 1]]),
        vec![0, 2, 3, 1]
    );
}
