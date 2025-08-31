pub fn square(s: u32) -> u64 {
    let mut sum = 1;

    for _ in 0..s - 1 {
        sum += sum;
    }

    sum
}

pub fn total() -> u128 {
    let mut sum = 1;

    for _ in 0..64 {
        sum += sum;
    }

    sum - 1
}
