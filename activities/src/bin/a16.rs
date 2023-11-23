// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker_assignment: Option<i32>,
}

fn main() {
    let l = Student {
        name: "John".to_owned(),
        locker_assignment: Some(32),
    };
    match l.locker_assignment {
        Some(locker_assignment) => {
            println!("student {} locker number {}", l.name, locker_assignment)
        }
        None => println!("student {} no locker", l.name),
    }
}
