// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main() {
    let mut store = HashMap::new();
    store.insert("chairs", 5);
    store.insert("beds", 3);
    store.insert("tables", 2);
    store.insert("couches", 0);
    for (name, amount) in store.iter() {
        if amount == &0 {
            println!("{} is out of stock", name);
            continue;
        }
        println!("{} {} in store", amount, name)
    }
    let total: i32 = store.values().sum();
    println!("{} total items in the store", total);
}
