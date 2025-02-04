#[allow(clippy::ptr_arg)]
pub fn reverse_string(s: &mut Vec<char>) {
    let (mut start, mut end) = (0, s.len() - 1);
    while start < end {
        s.swap(start, end);
        start += 1;
        end -= 1;
    }
}

#[test]
fn test_reverse_string() {
    let mut s = vec!['h', 'e', 'l', 'l', 'o'];
    reverse_string(&mut s);
    assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);

    let mut s = vec!['H', 'a', 'n', 'n', 'a', 'h'];
    reverse_string(&mut s);
    assert_eq!(s, vec!['h', 'a', 'n', 'n', 'a', 'H']);
}
