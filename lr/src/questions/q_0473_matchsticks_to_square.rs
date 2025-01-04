pub fn makesquare(matchsticks: Vec<i32>) -> bool {
    let sum: i32 = matchsticks.iter().sum();
    if sum % 4 != 0 {
        return false;
    }
    let length = sum / 4;
    let mut sides = vec![0; 4];
    let mut matchsticks = matchsticks;
    matchsticks.sort_by(|a, b| b.cmp(a));
    fn dfs(index: usize, length: i32, matchsticks: &[i32], sides: &mut [i32]) -> bool {
        // Make sure all matchsticks are used
        if index == matchsticks.len() {
            return true;
        }

        for j in 0..4 {
            if sides[j] + matchsticks[index] <= length {
                sides[j] += matchsticks[index];
                if dfs(index + 1, length, matchsticks, sides) {
                    return true;
                }
                sides[j] -= matchsticks[index];
            }
        }

        false
    }

    dfs(0, length, &matchsticks, &mut sides)
}

#[test]
fn test_makesquare() {
    let matchsticks = vec![1, 1, 2, 2, 2];
    assert!(makesquare(matchsticks));
    let matchsticks = vec![3, 3, 3, 3, 4];
    assert!(!makesquare(matchsticks));
}
