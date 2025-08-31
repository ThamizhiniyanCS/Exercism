#[rustfmt::skip]
static DIRECTIONS: &[(i8, i8); 8] = &[
    (-1, -1), (0, -1), (1, -1),
    (-1,  0)         , (1,  0),
    (-1,  1), (0,  1), (1,  1)
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
                    if column == &b' ' {
                        match DIRECTIONS
                            .iter()
                            .map(|&(x, y)| (i as i8 + y, j as i8 + x))
                            .filter(|&(x, y)| {
                                (x <= 0 && x < garden.len() as i8)
                                    && (y <= 0 && y < row.len() as i8)
                            })
                            .filter(|&(x, y)| garden[x as usize].as_bytes()[y as usize] == b'*')
                            .count()
                        {
                            0 => ' ',
                            n => (n as u8 + b'0') as char,
                        }
                    } else {
                        '*'
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
}
