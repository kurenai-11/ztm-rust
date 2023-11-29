// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing

#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

struct ShoesColor(Color);
impl ShoesColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
}
struct ShirtColor(Color);
impl ShirtColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
}
struct PantsColor(Color);
impl PantsColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

fn main() {
    let my_shirt = ShirtColor::new(Color::Black);
    let my_pants = PantsColor::new(Color::Yellow);
    let my_shoes = ShoesColor::new(Color::Custom("Magenta".to_owned()));
    println!("{:?}", my_shirt.0);
    println!("{:?}", my_pants.0);
    println!("{:?}", my_shoes.0);
}
