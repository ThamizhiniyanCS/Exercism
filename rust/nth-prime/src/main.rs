use nth_prime::*;

fn main() {
    let output = nth(5);
    let expected = 13;
    assert_eq!(output, expected);
}
