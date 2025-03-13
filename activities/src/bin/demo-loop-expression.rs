fn main() {
    loop {
        println!("hello!");
        break;
    }

    let mut i = 10;
    loop {
        println!("{:?}", i);
        i -= 1;
        if i == 0 {
            break;
        }
    }
    println!("Blast off!");
}
