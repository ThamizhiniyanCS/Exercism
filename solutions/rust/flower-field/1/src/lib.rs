#[rustfmt::skip]
const DIRECTIONS: [(i8, i8); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1, 0)          , (1, 0),
    (-1, 1),  (0, 1) , (1, 1)
];

pub fn annotate(garden: &[&str]) -> Vec<String> {
    garden
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.as_bytes()
                .iter()
                .enumerate()
                .map(|(j, column)| {
                    if column == &32u8 {
                        let mut count: u8 = 0;

                        for (x, y) in DIRECTIONS {
                            let a = i as i8 + y;
                            let b = j as i8 + x;

                            let r: usize = if a >= 0 && a < garden.len() as i8 {
                                a as usize
                            } else {
                                continue;
                            };
                            let c: usize = if b >= 0 && b < row.len() as i8 {
                                b as usize
                            } else {
                                continue;
                            };

                            if &garden[r][c..c + 1] == "*" {
                                count += 1
                            }
                        }

                        if count > 0 {
                            (count + b'0') as char
                        } else {
                            ' '
                        }
                    } else {
                        '*'
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
}
