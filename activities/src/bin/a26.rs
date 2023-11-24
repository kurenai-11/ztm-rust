// Topic: External crates
//
// Requirements:
// * Display the current date and time
//
// Notes:
// * Use the `chrono` crate to work with time
// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
//   for examples on how to create custom time formats

use chrono::prelude::Utc;

fn main() {
    let cur_time = Utc::now().to_rfc3339();
    println!("{}", cur_time);
}
