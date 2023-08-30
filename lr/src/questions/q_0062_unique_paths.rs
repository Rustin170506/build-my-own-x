pub fn unique_paths(m: i32, n: i32) -> i32 {
    let mut row = vec![1; n as usize];

    for _ in 0..m - 1 {
        let mut new_row = vec![1; n as usize];

        for j in (0..=(n - 2)).rev() {
            let j = j as usize;
            new_row[j] = new_row[j + 1] + row[j];
        }

        row = new_row;
    }

    row[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_paths() {
        assert_eq!(unique_paths(3, 7), 28);
        assert_eq!(unique_paths(3, 2), 3);
        assert_eq!(unique_paths(7, 3), 28);
        assert_eq!(unique_paths(3, 3), 6);
    }
}
