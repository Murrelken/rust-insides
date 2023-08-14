pub fn ith_fibonacci(mut i: u32) -> u32 {
    if i == 1 {
        return 0;
    } else if i == 2 {
        return 1;
    }

    let mut cur = 1;
    let mut prev = 1;

    i -= 2;

    while i > 0 {
        let temp_cur = cur;
        cur += prev;
        prev = temp_cur;
        i -= 1;
    }

    cur
}
