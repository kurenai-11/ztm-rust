// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

use std::io;

enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

fn get_power_state(input: &str) -> Option<PowerState> {
    match input.trim().to_lowercase().as_str() {
        "off" => Some(PowerState::Off),
        "sleep" => Some(PowerState::Sleep),
        "reboot" => Some(PowerState::Reboot),
        "shutdown" => Some(PowerState::Shutdown),
        "hibernate" => Some(PowerState::Hibernate),
        _ => None,
    }
}

fn print_power_message(power_state: Option<PowerState>) {
    if let Some(state) = power_state {
        match state {
            PowerState::Off => println!("it's off"),
            PowerState::Hibernate => println!("hibernated"),
            PowerState::Reboot => println!("about to be rebooten"),
            PowerState::Shutdown => println!("shutting down"),
            PowerState::Sleep => println!("in sleep"),
        }
    } else {
        println!("unknown input")
    }
}

fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    let power_state = get_power_state(&buf);
    print_power_message(power_state);
    Ok(())
}
