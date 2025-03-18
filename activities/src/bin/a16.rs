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
    locker: Option<i32>,
}
fn main() {
    let student1 = Student {
        name: "Jason Lee Scott".to_owned(),
        locker: Some(123),
    };

    println!("student: {:?}", student1.name);
    match student1.locker {
        Some(num) => println!("locker: {}", num),
        None => println!("No locker assigned")
    }
}

