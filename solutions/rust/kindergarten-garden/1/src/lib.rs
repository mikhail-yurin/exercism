use std::collections::HashMap;

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let mut plants_of_student: Vec<&'static str> = Vec::new();

    let students = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];

    let plants_map: HashMap<char, &str> = vec![
        ('G', "grass"),
        ('C', "clover"),
        ('R', "radishes"),
        ('V', "violets"),
    ]
    .into_iter()
    .collect();

    let student_position = students.iter().position(|&x| x == student).unwrap();

    let parts: Vec<&str> = diagram.split('\n').collect();
    let top = parts[0];
    let bottom = parts[1];

    for char in top[student_position * 2..student_position * 2 + 2].chars() {
        plants_of_student.push(plants_map[&char]);
    }
    for char in bottom[student_position * 2..student_position * 2 + 2].chars() {
        plants_of_student.push(plants_map[&char]);
    }
    dbg!(&plants_of_student);

    plants_of_student
}
