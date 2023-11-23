// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

type Coordinates = (f64, f64);

fn return_tuple() -> Coordinates {
    return (11.1, 22.3);
}

fn main() {
    let (_, y) = return_tuple();
    if y > 5.0 {
        println!("y>5")
    } else if y == 5.0 {
        println!("y==5")
    } else {
        println!("y<5")
    }
}
