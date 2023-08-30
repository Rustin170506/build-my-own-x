use std::collections::HashMap;

pub fn partition_labels(s: String) -> Vec<i32> {
    let mut last_indexes = HashMap::new();

    for (i, c) in s.chars().enumerate() {
        last_indexes.insert(c, i);
    }

    let mut result = vec![];
    let mut size = 0;
    let mut end = 0;
    for (i, c) in s.chars().enumerate() {
        size += 1;
        end = *last_indexes.get(&c).unwrap().max(&end);

        if i == end {
            result.push(size);
            size = 0;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_labels() {
        assert_eq!(
            vec![9, 7, 8],
            partition_labels("ababcbacadefegdehijhklij".to_string())
        );
    }
}
