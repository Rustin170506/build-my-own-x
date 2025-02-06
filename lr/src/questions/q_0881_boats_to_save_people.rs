pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
    let mut people = people;
    people.sort();
    let (mut l, mut r) = (0, people.len() - 1);
    let mut result = 0;
    while l < r {
        if people[l] + people[r] <= limit {
            result += 1;
            l += 1;
            r -= 1;
            continue;
        }
        result += 1;
        r -= 1;
    }
    if l == r {
        result += 1;
    }
    result
}

#[test]
fn test_num_rescue_boats() {
    let people = vec![1, 2];
    let limit = 3;
    assert_eq!(num_rescue_boats(people, limit), 1);
    let people = vec![3, 2, 2, 1];
    let limit = 3;
    assert_eq!(num_rescue_boats(people, limit), 3);
    let people = vec![3, 5, 3, 4];
    let limit = 5;
    assert_eq!(num_rescue_boats(people, limit), 4);
}
