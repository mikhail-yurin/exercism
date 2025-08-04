use std::collections::HashMap;

#[derive(Default)]
pub struct School {
    grades_with_students: HashMap<String, u32>,
}

impl School {
    pub fn new() -> School {
        School::default()
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.grades_with_students
            .entry(student.to_string())
            .or_insert_with(|| grade);
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grades_with_students
            .iter()
            .fold(vec![], |mut list, (_, students_grade)| {
                if !list.contains(students_grade) {
                    list.push(*students_grade);
                    list.sort();
                }
                list
            })
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut result: Vec<String> = self
            .grades_with_students
            .iter()
            .filter_map(|(student, students_grade)| {
                if students_grade == &grade {
                    Some(student.to_string())
                } else {
                    None
                }
            })
            .collect();

        result.sort();

        result
    }
}
