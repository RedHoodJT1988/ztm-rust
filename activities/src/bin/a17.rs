// Topic: Browsing standard library documentation
//
// Requirements:
// * Print a string in lowercase and uppercase
//
// Notes:
// * Utilize standard library functionality to
//   transform the string to lowercase and uppercase
// * Use 'rustup doc' in a terminal to open the standard library docs
// * Navigate to the API documentation section
// * Search for functionality to transform a string (or str)
//   to uppercase and lowercase
//   * Try searching for: to_uppercase, to_lowercase

fn main() {
    let lower_case = "HELLO, WORLD!".to_lowercase();
    let upper_case = "hello, world!".to_uppercase();
    println!("lower case: {}", lower_case);
    println!("upper case: {}", upper_case);
}

