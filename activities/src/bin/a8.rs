// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    MountainDew,
    BajaBlast,
    Coke,
    Pepsi
}

struct Drink {
    flavor: Flavor,
    fluid_ounces: f64,
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::MountainDew => println!("Mountain Dew"),
        Flavor::BajaBlast => println!("Baja Blast"),
        Flavor::Coke => println!("Coke"),
        Flavor::Pepsi => println!("Pepsi"),
    }
    println!("oz: {:?}", drink.fluid_ounces);
}
fn main() {
    let favorite_soda = Drink {
        flavor: Flavor::BajaBlast,
        fluid_ounces: 32.0,
    };

    let least_favorite_soda = Drink {
        flavor: Flavor::Coke,
        fluid_ounces: 8.0,
    };

    print_drink(favorite_soda);
    print_drink(least_favorite_soda);
}

