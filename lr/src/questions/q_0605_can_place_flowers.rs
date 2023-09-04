pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut new_flowerbed = [vec![0], flowerbed, vec![0]].concat();
    let mut n = n;

    for i in 1..new_flowerbed.len() - 1 {
        if new_flowerbed[i] == 0 && new_flowerbed[i - 1] == 0 && new_flowerbed[i + 1] == 0 {
            new_flowerbed[i] = 1;
            n -= 1;
        }
    }

    n <= 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_place_flowers() {
        assert!(can_place_flowers(vec![1, 0, 0, 0, 1], 1));
        assert!(!can_place_flowers(vec![1, 0, 0, 0, 1], 2));
    }
}
