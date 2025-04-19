use std::collections::HashSet;

pub fn total_n_queens(n: i32) -> i32 {
    let mut col = HashSet::new();
    let mut pos_diag = HashSet::new();
    let mut neg_diag = HashSet::new();
    let mut result = 0;
    fn backtrack(
        row: i32,
        col: &mut HashSet<i32>,
        pos_diag: &mut HashSet<i32>,
        neg_diag: &mut HashSet<i32>,
        n: i32,
        result: &mut i32,
    ) {
        if row == n {
            *result += 1;
            return;
        }
        for c in 0..n {
            if col.contains(&c) || pos_diag.contains(&(row + c)) || neg_diag.contains(&(row - c)) {
                continue;
            }

            col.insert(c);
            pos_diag.insert(row + c);
            neg_diag.insert(row - c);
            backtrack(row + 1, col, pos_diag, neg_diag, n, result);
            col.remove(&c);
            pos_diag.remove(&(row + c));
            neg_diag.remove(&(row - c));
        }
    }

    backtrack(0, &mut col, &mut pos_diag, &mut neg_diag, n, &mut result);
    result
}

#[test]
fn test_total_n_queens() {
    assert_eq!(total_n_queens(1), 1);
    assert_eq!(total_n_queens(2), 0);
    assert_eq!(total_n_queens(3), 0);
    assert_eq!(total_n_queens(4), 2);
    assert_eq!(total_n_queens(5), 10);
}
