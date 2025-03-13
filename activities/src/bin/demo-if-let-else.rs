
#![allow(dead_code)]

enum Color {
    Teal,
    Black,
    Orange,
}

fn main() {
    let maybe_user = Some("Jerry");
    match maybe_user {
        Some(user) => println!("user={user:?}"),
        None => println!("no user"),
    }

    if let Some(user) = maybe_user {
        println!("user={user:?}");
    } else {
        println!("no user");
    }

    let teal = Color::Teal;
    if let Color::Teal = teal {
        println!("its teal!");
    } else {
        println!("its not teal");
    }
}
