pub fn decode_string(s: String) -> String {
    let mut stack = vec![];
    for c in s.chars() {
        if c != ']' {
            stack.push(c.to_string());
        } else {
            // Get the string part
            let mut curr_str = String::new();
            while let Some(top) = stack.last() {
                if top == "[" {
                    stack.pop();
                    break;
                }
                curr_str = stack.pop().unwrap() + &curr_str;
            }

            // Get the number part
            let mut num_str = String::new();
            while let Some(top) = stack.last() {
                if !top.chars().next().unwrap().is_ascii_digit() {
                    break;
                }
                num_str = stack.pop().unwrap() + &num_str;
            }

            let k = num_str.parse::<usize>().unwrap();
            stack.push(curr_str.repeat(k));
        }
    }

    stack.join("")
}

#[test]
fn test_decode_string() {
    assert_eq!(
        decode_string("3[a]2[bc]".to_string()),
        "aaabcbc".to_string()
    );
    assert_eq!(
        decode_string("3[a2[c]]".to_string()),
        "accaccacc".to_string()
    );
    assert_eq!(
        decode_string("2[abc]3[cd]ef".to_string()),
        "abcabccdcdcdef".to_string()
    );
}
