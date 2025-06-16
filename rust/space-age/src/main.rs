use space_age::*;

fn main() {
    let seconds = 1_000_000_000;
    let duration = Duration::from(seconds);
    let output = Earth::years_during(&duration);
    let expected = 31.69;
    println!("{} = {}", expected, output);

    let seconds = 1_821_023_456;
    let duration = Duration::from(seconds);
    let output = Neptune::years_during(&duration);
    let expected = 0.35;
    println!("{} = {}", expected, output);
}
