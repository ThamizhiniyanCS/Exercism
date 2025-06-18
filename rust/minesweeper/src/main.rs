use minesweeper::*;

fn main() {
    let input = &[" * * "];
    let expected = &["1*2*1"];
    let actual = annotate(input);
    assert_eq!(actual, expected);
}
