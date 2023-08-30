use std::vec;

pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut result = vec![vec![1]];
    for _ in 0..num_rows - 1 {
        let mut temp = vec![0];
        temp.append(&mut result.last().unwrap().clone());
        temp.push(0);
        let mut row = vec![];
        for j in 0..(temp.len() - 1) {
            row.push(temp[j] + temp[j + 1])
        }
        result.push(row);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        assert_eq!(generate(1), vec![vec![1]]);
        assert_eq!(
            generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
    }
}
