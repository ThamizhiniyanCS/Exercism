pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 1 {
        return format!("And all for the want of a {}.", list[0]);
    };

    list.windows(2)
        .map(|window| format!("For want of a {} the {} was lost.", window[0], window[1]))
        .collect::<Vec<String>>()
        .join("\n")
        + format!("\nAnd all for the want of a {}.", list[0]).as_str()
}
