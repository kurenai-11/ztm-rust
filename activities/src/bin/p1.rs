// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.

use std::{
    collections::HashMap,
    io::{self, Error},
};

struct Bill {
    name: String,
    amount: f64,
}

enum MainMenuChoice {
    AddBill,
    EditBill,
    DeleteBill,
    Exit,
}

fn show_menu(bills: &Vec<Bill>) {
    println!("Hey hey hello this is a bill app ya.");
    if bills.is_empty() {
        println!("Looks like you don't have any bills yet, eh?");
    } else {
        println!("Current bills information leh:");
        for bill in bills {
            println!("name: {}, amount: {} leh.", bill.name, bill.amount);
        }
    }
    let choices = print_main_choices(!bills.is_empty());
    let choice = read_user_choice(&choices);
    if let Ok(choice) = choice {
        match choice {
            MainMenuChoice::AddBill => println!("adding bill huh"),
            MainMenuChoice::Exit => println!("exiting huh"),
            _ => println!("other huh"),
        }
    }
}

fn print_main_choices(has_bills: bool) -> HashMap<i32, MainMenuChoice> {
    let mut counter = 1;
    let mut choices = HashMap::new();
    println!("{}. Add bill ah.", counter);
    choices.insert(counter, MainMenuChoice::AddBill);
    counter += 1;
    if has_bills {
        println!("{}. Edit bill uh?", counter);
        choices.insert(counter, MainMenuChoice::EditBill);
        counter += 1;
        println!("{}. Delete bill huh?", counter);
        choices.insert(counter, MainMenuChoice::DeleteBill);
        counter += 1;
    }
    println!("{}. Exit the program lah.", counter);
    choices.insert(counter, MainMenuChoice::Exit);
    choices
}

fn read_user_choice<T>(choices: &HashMap<i32, T>) -> Result<&T, String> {
    let mut buf = String::new();
    let result = io::stdin().read_line(&mut buf);
    if let Err(e) = result {
        return Err(format!("{}", e));
    }
    let choice = buf.trim().parse::<i32>();
    match choice {
        Ok(choice) => match choices.get(&choice) {
            Some(choice) => Ok(choice),
            None => Err("couldn't find the choice in the choices list".to_owned()),
        },
        Err(e) => Err(format!("{}", e)),
    }
}

fn main() {
    let bills = vec![];
    show_menu(&bills);
}
