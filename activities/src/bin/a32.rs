// Topic: Lifetimes & Structures
//
// Requirements:
// * Display just the names and titles of persons from the mock-data.csv file
// * The names & titles must be stored in a struct separately from the mock
//   data for potential later usage
// * None of the mock data may be duplicated in memory
//
// Notes:
// * The mock data has already been loaded with the include_str! macro, so all functionality
//   must be implemented using references/borrows

const MOCK_DATA: &str = include_str!("mock-data.csv");

struct Person<'a> {
    name: &'a str,
    title: &'a str,
}

fn main() {
    MOCK_DATA
        .lines()
        .skip(1)
        .map(|l| {
            let parts: Vec<&str> = l.split(',').collect();
            Person {
                name: parts[1],
                title: parts[4],
            }
        })
        .for_each(|person| println!("name: {}, title: {}", person.name, person.title));
}
