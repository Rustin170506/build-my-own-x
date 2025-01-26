struct NumMatrix {
    matrix: Vec<Vec<i32>>,
    sum_matrix: Vec<Vec<i32>>,
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let mut sum_matrix = vec![vec![0; matrix[0].len()]; matrix.len()];
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                sum_matrix[i][j] = matrix[i][j];
                if i > 0 {
                    sum_matrix[i][j] += sum_matrix[i - 1][j];
                }
                if j > 0 {
                    sum_matrix[i][j] += sum_matrix[i][j - 1];
                }
                if i > 0 && j > 0 {
                    sum_matrix[i][j] -= sum_matrix[i - 1][j - 1];
                }
            }
        }

        Self { matrix, sum_matrix }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let row1 = row1 as usize;
        let col1 = col1 as usize;
        let row2 = row2 as usize;
        let col2 = col2 as usize;
        let mut res = self.sum_matrix[row2][col2];
        if row1 > 0 {
            res -= self.sum_matrix[row1 - 1][col2];
        }
        if col1 > 0 {
            res -= self.sum_matrix[row2][col1 - 1];
        }
        if row1 > 0 && col1 > 0 {
            res += self.sum_matrix[row1 - 1][col1 - 1];
        }
        res
    }
}

#[test]
fn test_num_matrix() {
    let matrix = vec![
        vec![3, 0, 1, 4, 2],
        vec![5, 6, 3, 2, 1],
        vec![1, 2, 0, 1, 5],
        vec![4, 1, 0, 1, 7],
        vec![1, 0, 3, 0, 5],
    ];
    let num_matrix = NumMatrix::new(matrix);
    let res = num_matrix.sum_region(2, 1, 4, 3);
    assert_eq!(res, 8);
    let res = num_matrix.sum_region(1, 1, 2, 2);
    assert_eq!(res, 11);
    let res = num_matrix.sum_region(1, 2, 2, 4);
    assert_eq!(res, 12);
}
