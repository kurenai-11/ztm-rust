// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

trait Perimeter {
    fn get_perimeter(&self) -> f64;
}

struct Square {
    side: f64,
}

struct Triangle {
    sides: (f64, f64, f64),
}

impl Perimeter for Square {
    fn get_perimeter(&self) -> f64 {
        self.side * 4.0
    }
}

impl Perimeter for Triangle {
    fn get_perimeter(&self) -> f64 {
        let (a, b, c) = self.sides;
        a + b + c
    }
}

fn print_perimeter(shape: &impl Perimeter) {
    println!("perimeter is {}", shape.get_perimeter())
}

fn main() {
    print_perimeter(&Square { side: 4.0 });
    print_perimeter(&Triangle {
        sides: (3.0, 4.0, 5.0),
    });
}
