#[allow(clippy::needless_range_loop)]
pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let (rows, cols) = (matrix.len(), matrix[0].len());
    let mut row_zero = false;

    for r in 0..rows {
        for c in 0..cols {
            if matrix[r][c] == 0 {
                matrix[0][c] = 0;
                if r > 0 {
                    matrix[r][0] = 0;
                } else {
                    row_zero = true;
                }
            }
        }
    }

    for r in 1..rows {
        for c in 1..cols {
            if matrix[0][c] == 0 || matrix[r][0] == 0 {
                matrix[r][c] = 0;
            }
        }
    }

    if matrix[0][0] == 0 {
        for r in 0..rows {
            matrix[r][0] = 0;
        }
    }
    if row_zero {
        for c in 0..cols {
            matrix[0][c] = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_zeroes() {
        let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1],]);

        let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        set_zeroes(&mut matrix);
        assert_eq!(
            matrix,
            vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0],]
        );
    }
}
