// Topic: Generics & Structures
//
// Requirements:
// * Create a Vehicle structure that is generic over traits Body and Color
// * Create structures for vehicle bodies and vehicle colors and implement the
//   Body and Color traits for these structures
// * Implement a 'new' function for Vehicle that allows it to have any body
//   and any color
// * Create at least two different vehicles in the main function and print their
//   info
//
// Notes:
// * Examples of car bodies can be Truck, Car, Scooter
// * Examples of colors could be red, white, black
// * It is not necessary to have data fields or function implementations
//   for the vehicle bodies/colors

trait Body: std::fmt::Debug {}
trait Color: std::fmt::Debug {}

#[derive(Debug)]
struct Truck {}
impl Body for Truck {}

#[derive(Debug)]
struct Car {}
impl Body for Car {}

#[derive(Debug)]
struct Scooter {}
impl Body for Scooter {}

#[derive(Debug)]
struct Red {}
impl Color for Red {}

#[derive(Debug)]
struct White {}
impl Color for White {}

#[derive(Debug)]
struct Black {}
impl Color for Black {}

struct Vehicle<T: Body, U: Color> {
    name: String,
    body: T,
    color: U,
}

impl<T: Body, U: Color> Vehicle<T, U> {
    fn new(name: &str, body: T, color: U) -> Self {
        Vehicle {
            name: name.to_owned(),
            body,
            color,
        }
    }

    fn print_info(&self) {
        println!(
            "this is {:?}, body: {:?}, color: {:?}",
            self.name, self.body, self.color
        )
    }
}

fn main() {
    let v1 = Vehicle::new("land rover", Car {}, White {});
    let v2 = Vehicle::new("jenny", Scooter {}, Red {});
    v1.print_info();
    v2.print_info();
}
