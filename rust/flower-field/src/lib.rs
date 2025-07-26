#[rustfmt::skip]
static DIRECTIONS: &[(i8, i8); 8] = &[
    (-1, -1), (0, -1), (1, -1),
    (-1,  0)         , (1,  0),
    (-1,  1), (0,  1), (1,  1)
];

pub fn annotate(garden: &[&str]) -> Vec<String> {
    let columns: i8 = garden.len() as i8;
    let rows: i8 = garden[0].len() as i8;

    (0..columns)
        .map(|i| {
            (0..rows)
                .map(|j| {
                    if garden[i as usize].as_bytes()[j as usize] == b' ' {
                        match DIRECTIONS
                            .iter()
                            .map(|&(x, y)| (i + y, j + x))
                            .filter(|&(x, y)| (0 <= x && x < columns) && (0 <= y && y < rows))
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
