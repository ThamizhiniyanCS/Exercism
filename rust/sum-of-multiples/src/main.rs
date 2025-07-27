use sum_of_multiples::*;

fn main() {
    let factors = &[3];
    let limit = 7;
    let output = sum_of_multiples(limit, factors);
    let expected = 9;
    assert_eq!(output, expected);
}
