fn sub(a: i32, b: i32) -> i32 {
    a - b
}

fn main() {
    let sum = 3 + 3;
    let difference = 10 - 4;
    let product = 4 * 4;
    let quotient = 100 / 5;

    let six = sub(9, 3);

    let remainder = 6 % 3;
    let remainder2 = 6 % 4;

    println!("sum: {}", sum);
    println!("difference: {}", difference);
    println!("product: {}", product);
    println!("quotient: {}", quotient);
    println!("six: {}", six);
    println!("remainder: {}", remainder);
    println!("remainder2: {}", remainder2);
}
