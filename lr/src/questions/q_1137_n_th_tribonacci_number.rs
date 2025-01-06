pub fn tribonacci(n: i32) -> i32 {
    let mut t = [0, 1, 1];
    if n < 3 {
        return t[n as usize];
    }
    for _ in 3..n + 1 {
        (t[0], t[1], t[2]) = (t[1], t[2], t.iter().sum())
    }
    t[2]
}

#[test]
fn test_tribonacci() {
    let n = 25;
    let res = tribonacci(n);
    assert_eq!(res, 1389537);
}
