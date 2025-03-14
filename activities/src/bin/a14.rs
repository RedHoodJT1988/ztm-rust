// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    name: String,
    fav_color: String,
    age: i32
}

fn print(data: &str) {
    println!("{:?}", data);
}
fn main() {
    let people = vec![
        Person { name: String::from("Alice"), fav_color: String::from("red"), age: 10 },
        Person { name: String::from("Bobby"), fav_color: String::from("blue"), age: 11 },
        Person { name: String::from("Charlie"), fav_color: String::from("green"), age: 9 },
    ];

    for person in &people {
        if person.age <= 10 {
            print(&person.name);
            print(&person.fav_color);
        }
    }
}

