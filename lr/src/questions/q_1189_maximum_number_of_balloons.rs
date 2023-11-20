use std::collections::HashMap;

pub fn max_number_of_balloons(text: String) -> i32 {
    let balloon_character_counts: HashMap<char, u64> =
        vec![('b', 1), ('a', 1), ('l', 2), ('o', 2), ('n', 1)]
            .into_iter()
            .collect();

    let text_character_counts: HashMap<char, u64> =
        text.chars().fold(HashMap::new(), |mut counts, character| {
            *counts.entry(character).or_insert(0) += 1;
            counts
        });

    let mut max_balloon_count = text.len() as i32;

    for (character, count) in balloon_character_counts {
        if let Some(text_count) = text_character_counts.get(&character) {
            max_balloon_count = i32::min(max_balloon_count, (text_count / count) as i32);
        } else {
            return 0;
        }
    }

    max_balloon_count
}

#[test]
fn test_max_number_of_balloons() {
    assert_eq!(max_number_of_balloons("nlaebolko".to_string()), 1);
    assert_eq!(max_number_of_balloons("loonbalxballpoon".to_string()), 2);
    assert_eq!(max_number_of_balloons("leetcode".to_string()), 0);
}
