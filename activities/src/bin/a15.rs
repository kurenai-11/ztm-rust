// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Ticket {
    Backstage(i32, String),
    Vip(i32, String),
    Standard(i32),
}

fn main() {
    let b = Ticket::Backstage(50, "John".to_owned());
    let v = Ticket::Vip(35, "James".to_owned());
    let s = Ticket::Standard(20);
    let tickets = vec![b, v, s];
    for ticket in tickets {
        match ticket {
            Ticket::Backstage(price, name) => {
                println!("backstage ticket with price {} and name {}", price, name)
            }
            Ticket::Vip(price, name) => {
                println!("vip ticket with price {} and name {}", price, name)
            }
            Ticket::Standard(price) => println!("standard ticket with price {}", price),
        }
    }
}
