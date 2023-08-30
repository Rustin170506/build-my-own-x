pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let (rows, cols) = (matrix.len(), matrix[0].len());

    let (mut top, mut bot) = (0_i32, (rows - 1) as i32);

    while top <= bot {
        let mid = (top + bot) / 2;
        if matrix[mid as usize][0] > target {
            bot = mid - 1;
        } else if matrix[mid as usize][cols - 1] < target {
            top = mid + 1;
        } else {
            break;
        }
    }

    if top > bot {
        return false;
    }

    let row = (top + bot) / 2;
    let (mut left, mut right) = (0, cols - 1);
    while left <= right {
        let mid = (left + right) / 2;
        match matrix[row as usize][mid].cmp(&target) {
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Equal => return true,
            std::cmp::Ordering::Greater => right = mid - 1,
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_window() {
        assert!(search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            3
        ));
        assert!(search_matrix(vec![vec![1, 3, 5, 7]], 3));
        assert!(search_matrix(vec![vec![3]], 3));
    }
}
