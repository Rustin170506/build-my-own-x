pub fn partition(s: String) -> Vec<Vec<String>> {
    let mut res = vec![];
    let mut part = vec![];

    fn dfs(s: &String, res: &mut Vec<Vec<String>>, part: &mut Vec<String>, i: usize) {
        if i >= s.len() {
            res.push(part.clone());
            return;
        }

        for j in i..s.len() {
            if is_pail(s, i, j) {
                part.push(s[i..j + 1].to_string());
                dfs(s, res, part, j + 1);
                part.pop().unwrap();
            }
        }
    }

    dfs(&s, &mut res, &mut part, 0);

    res
}

fn is_pail(s: &str, i: usize, j: usize) -> bool {
    let mut i = i;
    let mut j = j;
    while i < j {
        if s.chars().nth(i).unwrap() != s.chars().nth(j).unwrap() {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            partition("aab".to_owned()),
            vec![vec!["a", "a", "b"], vec!["aa", "b"]]
        )
    }
}
