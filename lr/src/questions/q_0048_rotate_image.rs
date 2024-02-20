#[allow(clippy::ptr_arg)]
pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let (mut l, mut r) = (0, matrix.len() - 1);

    while l < r {
        for i in 0..r - l {
            let (top, bottom) = (l, r);

            // Save the top left.
            let top_left = matrix[top][l + i];

            // Move bottom left into top left.
            matrix[top][l + i] = matrix[bottom - i][l];

            // Move bottom right into bottom left.
            matrix[bottom - i][l] = matrix[bottom][r - i];

            // Move top right into bottom right.
            matrix[bottom][r - i] = matrix[top + i][r];

            // Move top left into top right.
            matrix[top + i][r] = top_left;
        }

        r -= 1;
        l += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rotate() {
        let mut m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        rotate(&mut m);
        assert_eq!(m, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]])
    }
}
