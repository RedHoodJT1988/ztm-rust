// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

struct ShippingBox {
    dimensions: (i32, i32, i32),
    weight: i32,
    color: BoxColor,
}

#[derive(Debug)]
enum BoxColor {
    Brown,
    Black,
    White,
}

impl ShippingBox {
    fn new(dimensions: (i32, i32, i32), weight: i32, color: BoxColor) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }

    fn print_characteristics(&self) {
        println!("dimensions: {:?}", self.dimensions);
        println!("weight: {:?}", self.weight);
        println!("color: {:?}", self.color);
    }
}
fn main() {
    let box1 = ShippingBox::new((10, 10, 10), 5, BoxColor::Brown);
    box1.print_characteristics();
    let box2 = ShippingBox::new((20, 20, 20), 10, BoxColor::Black);
    box2.print_characteristics();
    let box3 = ShippingBox::new((30, 30, 30), 15, BoxColor::White);
    box3.print_characteristics();
}

