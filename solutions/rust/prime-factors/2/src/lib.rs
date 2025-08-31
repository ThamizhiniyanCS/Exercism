// NOTE :Generate the next prime number using cached prime numbers
fn find_next_prime_number(cache: &mut Vec<u64>) -> u64 {
    // NOTE: Start from one more than the last cached value
    let mut i = cache.last().copied().unwrap_or(1) + 1;

    loop {
        let prime_number_limit = (i as f64).sqrt() as u64;

        let is_prime = !cache
            .iter()
            .take_while(|&&p| p <= prime_number_limit)
            .any(|&p| i % p == 0);

        if is_prime {
            cache.push(i);
            return i;
        }

        i += 1
    }
}

pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    let mut primes: Vec<u64> = Vec::new();

    while n > 1 {
        let mut found = false;

        // NOTE: Try dividing with already known primes
        for &p in &primes {
            if n % p == 0 {
                factors.push(p);
                n /= p;
                found = true;
                break;
            }
        }

        // NOTE: Generate new primes if none found in already known primes
        if !found {
            loop {
                let p = find_next_prime_number(&mut primes);

                if n % p == 0 {
                    factors.push(p);
                    n /= p;
                    break;
                }
            }
        }
    }

    factors
}
