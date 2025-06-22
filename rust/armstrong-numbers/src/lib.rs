pub fn is_armstrong_number(num: u32) -> bool {
    let len = (num as f64).log10().floor() as u32 + 1;
    let mut n = num;
    let mut sum = 0;

    while n > 0 {
        let digit = n % 10;
        sum += digit.pow(len);
        n /= 10;
    }

    sum == num
}
