use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples: HashSet<u32> = HashSet::new();

    for each in factors {
        if each == &0u32 {
            continue;
        }

        let mut counter = 1;

        while each * counter < limit {
            multiples.insert(each * counter);
            counter += 1
        }
    }

    multiples.iter().sum::<u32>()
}
