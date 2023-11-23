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
    Cola,
    Fanta,
    Milk,
}

struct Drink {
    flavor: Flavor,
    amount: f64,
}

fn print_drink_info(drink: Drink) {
    match drink.flavor {
        Flavor::Cola => println!("Cola"),
        Flavor::Fanta => println!("Fanta"),
        Flavor::Milk => println!("Milk"),
    }
    println!("amount: {}", drink.amount)
}

fn main() {
    let drink = Drink {
        flavor: Flavor::Cola,
        amount: 64.5,
    };
    print_drink_info(drink)
}
