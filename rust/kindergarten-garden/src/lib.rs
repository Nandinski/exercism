const STUDENTS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let student_offset = STUDENTS
        .iter()
        .position(|&s| s == student)
        .map(|i| i * 2)
        .unwrap();

    diagram
        .lines()
        .flat_map(|garden_row| {
            garden_row[student_offset..student_offset + 2]
                .chars()
                .map(|plant_encoding| match plant_encoding {
                    'G' => "grass",
                    'C' => "clover",
                    'R' => "radishes",
                    'V' => "violets",
                    _ => panic!("Unexpected plant encoding {plant_encoding}"),
                })
        })
        .collect()
}
