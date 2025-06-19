use luhn::*;

fn main() {
    // is_valid("4539 3195 0343 6467");
    // is_valid("8273 1232 7352 0569");
    assert!(!is_valid("59%59"));
}
