#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn contains<T: PartialEq>(haystack: &[T], needle: &[T]) -> bool {
    if needle.is_empty() {
        return false;
    }

    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    use Comparison::*;

    match (first_list.len(), second_list.len()) {
        (_, _) if first_list == second_list => Equal,
        (_, 0) => Superlist,
        (0, _) => Sublist,
        (a, b) if a > b && contains(first_list, second_list) => Superlist,
        (a, b) if b > a && contains(second_list, first_list) => Sublist,
        (_, _) => Unequal,
    }
}
