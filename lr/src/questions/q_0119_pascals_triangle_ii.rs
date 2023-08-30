pub fn get_row(row_index: i32) -> Vec<i32> {
    let mut result = vec![vec![1]];
    for _ in 0..row_index + 1 {
        let mut temp = vec![0];
        temp.append(&mut result.last().unwrap().clone());
        temp.push(0);
        let mut row = vec![];
        for j in 0..(temp.len() - 1) {
            row.push(temp[j] + temp[j + 1])
        }
        result.push(row);
    }

    result[row_index as usize].clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_row() {
        assert_eq!(get_row(0), vec![1]);
        assert_eq!(get_row(1), vec![1, 1]);
        assert_eq!(get_row(3), vec![1, 3, 3, 1]);
    }
}
