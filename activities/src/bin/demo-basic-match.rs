fn main() {
    let some_bool = true;

    match some_bool {
        true => println!("its true"),
        false => println!("its false")
    }

    let some_int = 3;
    match some_int {
        1 => println!("its 1"),
        2 => println!("its 2"),
        3 => println!("its 3"),
        _ => println!("its something else")
    }
}
