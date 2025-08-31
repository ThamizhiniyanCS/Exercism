static STUDENTS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let diagram: Vec<String> = diagram.lines().map(String::from).collect::<Vec<String>>();
    let student_number = STUDENTS.iter().position(|&s| s == student).unwrap() + 1;
    let student_cups_positions = [
        (0, student_number * 2 - 2),
        (0, student_number * 2 - 1),
        (1, student_number * 2 - 2),
        (1, student_number * 2 - 1),
    ];

    student_cups_positions
        .map(|(x, y)| match &diagram[x][y..y + 1] {
            "G" => "grass",
            "C" => "clover",
            "R" => "radishes",
            "V" => "violets",
            _ => "",
        })
        .to_vec()
}
