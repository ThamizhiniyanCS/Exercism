pub fn square(s: u32) -> u64 {
    if !(1..=64).contains(&s) {
        panic!("Square must be between 1 to 64")
    }

    2_u64.pow(s - 1)
}

pub fn total() -> u64 {
    (2_u128.pow(64) - 1) as u64
}
