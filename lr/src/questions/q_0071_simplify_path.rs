use std::collections::VecDeque;

pub fn simplify_path(path: String) -> String {
    let mut stack = VecDeque::new();
    for x in path.split('/') {
        match x {
            ".." => {
                stack.pop_back();
            }
            "." | "" => {}
            _ => stack.push_back(x),
        }
    }

    format!(
        r#"/{}"#,
        stack.iter().cloned().collect::<Vec<_>>().join("/")
    )
}

#[test]
fn test_simplify_path() {
    assert_eq!(simplify_path("/home/".to_string()), "/home".to_string());
    assert_eq!(
        simplify_path("/home//foo/".to_string()),
        "/home/foo".to_string()
    );
    assert_eq!(simplify_path("/..//".to_string()), "/".to_string());
    assert_eq!(
        simplify_path("/.../a/../b/c/../d/./".to_string()),
        "/.../b/d".to_string()
    );
}
