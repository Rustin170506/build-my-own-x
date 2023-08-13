use std::collections::HashMap;

pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
    let mut map = HashMap::new();
    for i in hand.clone() {
        *map.entry(i).or_insert(0) += 1;
    }
    let mut hand = hand;
    hand.sort();
    for i in hand {
        if *map.get(&i).unwrap() == 0 {
            continue;
        }
        for j in 0..group_size {
            if let Some(v) = map.get_mut(&(i + j)) {
                if *v == 0 {
                    return false;
                }
                *v -= 1;
            } else {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_is_n_straight_hand() {
        assert_eq!(is_n_straight_hand(vec![1, 2, 3, 6, 2, 3, 4, 7, 8], 3), true);
        assert_eq!(is_n_straight_hand(vec![1, 2, 3, 4, 5], 4), false);
    }
}
