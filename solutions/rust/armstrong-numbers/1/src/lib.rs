pub fn is_armstrong_number(num: u32) -> bool {
    let nums: Vec<u32> = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    nums.iter().map(|n| n.pow(nums.len() as u32)).sum::<u32>() == num
}
