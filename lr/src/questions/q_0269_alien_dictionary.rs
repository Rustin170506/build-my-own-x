pub fn alien_order(words: Vec<String>) -> String {
    let mut graph = vec![vec![]; 26];
    let mut in_degree = [0; 26];
    let mut seen = [false; 26];
    let mut result = String::new();
    let mut queue = Vec::new();

    for word in &words {
        for c in word.chars() {
            seen[(c as u8 - b'a') as usize] = true;
        }
    }

    for i in 0..words.len() - 1 {
        let word1 = words[i].as_bytes();
        let word2 = words[i + 1].as_bytes();
        let len = word1.len().min(word2.len());
        for j in 0..len {
            if word1[j] != word2[j] {
                let c1 = (word1[j] - b'a') as usize;
                let c2 = (word2[j] - b'a') as usize;
                in_degree[c2] += 1;
                graph[c1].push(c2);
                break;
            }
        }
    }

    for i in 0..26 {
        if in_degree[i] == 0 && seen[i] {
            queue.push(i);
        }
    }

    while let Some(c) = queue.pop() {
        
        result.push((c as u8 + b'a') as char);
        for &next in &graph[c] {
            in_degree[next] -= 1;
            if in_degree[next] == 0 {
                queue.push(next);
            }
        }
    }

    if result.len() != seen.iter().filter(|&&x| x).count() {
        return String::new();
    }

    result
}

#[test]
fn test_alien_order() {
    let words = vec![
        "wrt".to_string(),
        "wrf".to_string(),
        "er".to_string(),
        "ett".to_string(),
        "rftt".to_string(),
    ];
    assert_eq!(alien_order(words), "wertf".to_string());
}
