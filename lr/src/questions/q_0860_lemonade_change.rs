pub fn lemonade_change(bills: Vec<i32>) -> bool {
    let (mut five, mut ten) = (0, 0);
    for bill in bills {
        match bill {
            5 => five += 1,
            10 => {
                if five < 1 {
                    return false;
                }
                ten += 1;
                five -= 1;
            }
            20 => {
                if five >= 1 && ten >= 1 {
                    five -= 1;
                    ten -= 1;
                } else if five >= 3 {
                    five -= 3;
                } else {
                    return false;
                }
            }
            _ => unreachable!(),
        }
    }

    true
}

#[test]
fn test_lemonade_change() {
    let bills = vec![5, 5, 5, 10, 20];
    let result = lemonade_change(bills);
    assert!(result);

    let bills = vec![5, 5, 10];
    let result = lemonade_change(bills);
    assert!(result);

    let bills = vec![10, 10];
    let result = lemonade_change(bills);
    assert!(!result);
}
