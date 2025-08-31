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

pub fn factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();

    if n == 1 {
        return factors;
    }

    let mut valid_prime_numbers: Vec<u64> = Vec::new();

    let mut res = n;

    loop {
        if res == 1 {
            break;
        }

        let mut is_factor_found = false;

        for prime_number in &valid_prime_numbers {
            if res % prime_number == 0 {
                factors.push(*prime_number);
                res /= prime_number;
                is_factor_found = true;
                break;
            }
        }

        if !is_factor_found {
            loop {
                let prime_number = find_next_prime_number(&mut valid_prime_numbers);

                if res % prime_number == 0 {
                    factors.push(prime_number);
                    res /= prime_number;
                    break;
                }
            }
        }
    }

    factors
}
