pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return vec![];
    }

    let mut result: Vec<Vec<String>> =
        vec![vec![" ".to_string(); minefield[0].len()]; minefield.len()];

    for (i, row) in minefield.iter().enumerate() {
        for (j, column) in row.as_bytes().iter().enumerate() {
            if *column == 42u8 {
                result[i][j] = "*".to_string();
                continue;
            }

            let mut count: u32 = 0;

            // up
            if i > 0 && &minefield[i - 1][j..j + 1] == "*" {
                count += 1;
            }
            // down
            if i + 1 < minefield.len() && &minefield[i + 1][j..j + 1] == "*" {
                count += 1;
            }
            // left
            if j > 0 && &minefield[i][j - 1..j] == "*" {
                count += 1;
            }
            // right
            if j + 1 < minefield[0].len() && &minefield[i][j + 1..j + 2] == "*" {
                count += 1;
            }
            // up left
            if i > 0 && j > 0 && &minefield[i - 1][j - 1..j] == "*" {
                count += 1;
            }
            // up right
            if i > 0 && j + 1 < minefield[0].len() && &minefield[i - 1][j + 1..j + 2] == "*" {
                count += 1;
            }
            // down left
            if i + 1 < minefield.len() && j > 0 && &minefield[i + 1][j - 1..j] == "*" {
                count += 1;
            }
            // down right
            if i + 1 < minefield.len()
                && j + 1 < minefield[0].len()
                && &minefield[i + 1][j + 1..j + 2] == "*"
            {
                count += 1;
            }

            if count == 0 {
                result[i][j] = " ".to_string();
            } else {
                result[i][j] = count.to_string();
            }
        }
    }

    result
        .into_iter()
        .map(|row| row.into_iter().collect::<String>())
        .collect::<Vec<String>>()
}
