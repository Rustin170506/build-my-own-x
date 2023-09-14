unsafe fn guess_number(n: i32) -> i32 {
    let (mut start, mut end) = (1, n);
    while start <= end {
        let mid = (start + end) / 2;
        match guess(mid) {
            -1 => end = mid - 1,
            1 => start = mid + 1,
            0 => return mid,
            _ => unreachable!(),
        }
    }
    0
}

fn guess(_n: i32) -> i32 {
    todo!()
}
