#[rustfmt::skip]
static DIRECTIONS: &[(i32, i32)] = &[
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1),           (0, 1),
    (1, -1),  (1, 0),  (1, 1),
];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let columns: i32 = minefield.len() as i32;
    let rows: i32 = minefield[0].len() as i32;

    (0..columns)
        .map(|i| {
            (0..rows)
                .map(|j| {
                    if minefield[i as usize].as_bytes()[j as usize] == b'*' {
                        '*'
                    } else {
                        match DIRECTIONS
                            .iter()
                            .map(|&(dx, dy)| (i + dx, j + dy))
                            .filter(|&(x, y)| (0 <= x && x < columns) && (0 <= y && y < rows))
                            .filter(|&(x, y)| minefield[x as usize].as_bytes()[y as usize] == b'*')
                            .count()
                        {
                            0 => ' ',
                            n => (n as u8 + b'0') as char,
                        }
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
}
