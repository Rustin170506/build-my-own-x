pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    let mut stack = vec![];
    for asteroid in asteroids {
        let mut asteroid = asteroid;
        while !stack.is_empty() && asteroid < 0 && *stack.last().unwrap() > 0 {
            let diff: i32 = asteroid + *stack.last().unwrap();
            match diff.cmp(&0) {
                std::cmp::Ordering::Less => {
                    stack.pop();
                }
                std::cmp::Ordering::Equal => {
                    asteroid = 0;
                    stack.pop();
                }
                std::cmp::Ordering::Greater => {
                    asteroid = 0;
                }
            }
        }
        if asteroid != 0 {
            stack.push(asteroid);
        }
    }
    stack
}

#[test]
fn test_asteroid_collision() {
    let asteroids = vec![5, 10, -5];
    assert_eq!(asteroid_collision(asteroids), vec![5, 10]);
    let asteroids = vec![8, -8];
    assert_eq!(asteroid_collision(asteroids), vec![]);
    let asteroids = vec![10, 2, -5];
    assert_eq!(asteroid_collision(asteroids), vec![10]);
    let asteroids = vec![-2, -1, 1, 2];
    assert_eq!(asteroid_collision(asteroids), vec![-2, -1, 1, 2]);
}
