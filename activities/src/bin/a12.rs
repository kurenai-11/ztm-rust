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

type Dimensions = (f64, f64, f64);

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

struct ShippingBox {
    dimensions: Dimensions,
    color: Color,
    weight: f64,
}

impl ShippingBox {
    fn new(dimensions: Dimensions, color: Color, weight: f64) -> Self {
        Self {
            dimensions,
            color,
            weight,
        }
    }
    fn print_characteristics(&self) {
        let (x, y, z) = self.dimensions;
        println!("x: {}, y: {}, z: {}", x, y, z);
        println!("color: {:?}, weight: {}", self.color, self.weight);
    }
}

fn main() {
    let some_box = ShippingBox::new((0.0, 0.0, 0.0), Color::Blue, 0.0);
    some_box.print_characteristics()
}
