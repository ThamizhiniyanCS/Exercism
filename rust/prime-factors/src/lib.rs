pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();

    let mut i = 2;

    while n > 1 {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
        i += 1;
    }

    factors
}
