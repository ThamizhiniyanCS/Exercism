pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }

    let mut num: u32 = 3;
    let mut position_value: (u32, u32) = (0, 0);

    loop {
        if position_value.0 == n {
            break;
        }

        // NOTE: Checking if `num` is not even
        if num % 2 != 0 {
            let mut is_valid: bool = true;

            for i in 3..=(num / 2) {
                if num % i == 0 {
                    is_valid = false;
                    break;
                }
            }

            if is_valid {
                position_value.0 += 1;
                position_value.1 = num;
            }
        }

        num += 1
    }

    position_value.1
}
