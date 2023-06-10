pub fn my_pow(x: f64, n: i32) -> f64 {
    fn helper(x: f64, n: i32) -> f64 {
        if x == 0.0 {
            return 0.0;
        }

        if n == 0 {
            return 1.0;
        }

        let mut res = helper(x, n / 2);
        res = res * res;
        if n % 2 == 0 {
            res
        } else {
            res * x
        }
    }

    let res = helper(x, i32::abs(n));

    if n >= 0 {
        res
    } else {
        1.0 / res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_pow() {
        assert_eq!(my_pow(2.0, 2), 4.0);
    }
}
