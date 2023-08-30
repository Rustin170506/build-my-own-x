pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut res = vec![];
    let (mut left, mut right) = (0_i32, (matrix[0].len() - 1) as i32);
    let (mut top, mut bottom) = (0_i32, (matrix.len() - 1) as i32);

    while left <= right && top <= bottom {
        for i in left..=right {
            res.push(matrix[top as usize][i as usize]);
        }
        top += 1;

        for i in top..=bottom {
            res.push(matrix[i as usize][right as usize]);
        }
        right -= 1;

        if !(left <= right && top <= bottom) {
            break;
        }

        for i in (left..=right).rev() {
            res.push(matrix[bottom as usize][i as usize]);
        }
        bottom -= 1;

        for i in (top..=bottom).rev() {
            res.push(matrix[i as usize][left as usize]);
        }
        left += 1;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_054() {
        assert_eq!(
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5],
            spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9],])
        );
        assert_eq!(
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7],
            spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12],
            ])
        );
    }
}
