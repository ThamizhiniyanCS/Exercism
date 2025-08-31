use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn arr_contains<T>(arr_1: &[T], arr_2: &[T]) -> bool
where
    T: Display,
    T: Eq,
    T: PartialEq,
{
    for (i, value_1) in arr_1.iter().enumerate() {
        let mut is_sublist: bool = true;

        for (j, value_2) in arr_2.iter().enumerate() {
            if j == 0 && value_1 != value_2 {
                is_sublist = false;
                break;
            }

            if i + j < arr_1.len() && arr_1[i + j] != *value_2 {
                is_sublist = false;
                break;
            }
        }

        if is_sublist {
            return true;
        }
    }

    false
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }

    if first_list.len() > second_list.len() && arr_contains(first_list, second_list) {
        return Comparison::Superlist;
    }

    if second_list.len() > first_list.len() && arr_contains(second_list, first_list) {
        return Comparison::Sublist;
    }

    Comparison::Unequal
}
