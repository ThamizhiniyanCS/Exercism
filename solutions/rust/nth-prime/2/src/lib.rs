pub fn nth(n: u32) -> u32 {
    // NOTE: Caching valid prime numbers
    let mut prime_numbers: Vec<u32> = Vec::new();

    (2..)
        .filter(|&num| {
            // NOTE: Calculating the square root of the `num`.
            let limit = (num as f64).sqrt() as u32;

            let is_prime = !prime_numbers
                .iter()
                // NOTE: Filtering out prime numbers that are <= limit based on the following
                // theorem: [Reference: https://brilliant.org/wiki/prime-numbers/]
                // If `n` is a composite number, then it must be divisible by a prime `p` such that `p` <= square root of `n`
                .take_while(|&&p| p <= limit)
                // NOTE: Checking for divisibility only against previously found primes
                .any(|&p| num % p == 0);

            if is_prime {
                prime_numbers.push(num);
            }

            is_prime
        })
        .nth(n as usize)
        .unwrap()
}
