// Topic: Trait Objects
//
// Summary:
//   A contractor wants a program that can sum the cost of materials based
//   on how many square meters are required for a job.
//
// Requirements:
// * Calculate multiple material types with different costs
// * Must be able to process a list of varying materials
// * Material types and cost includes:
//   * Carpet - $10 per square meter
//   * Tile - $15 per square meter
//   * Wood - $20 per square meter
// * Square meters must be taken into account
//
// Notes:
// * Create a trait that can be used to retrieve the cost of a material
// * Create trait objects and store them in a vector for processing
// * Use a function to calculate the total cost
// * Process at least 3 different materials

use std::vec;

trait Material {
    fn cost(&self) -> f64;
}

struct Carpet {
    amount: f64,
}
impl Material for Carpet {
    fn cost(&self) -> f64 {
        self.amount * 10.0
    }
}
struct Tile {
    amount: f64,
}
impl Material for Tile {
    fn cost(&self) -> f64 {
        self.amount * 15.0
    }
}
struct Wood {
    amount: f64,
}
impl Material for Wood {
    fn cost(&self) -> f64 {
        self.amount * 20.0
    }
}

fn calculate_cost(materials: &[Box<dyn Material>]) -> f64 {
    materials.iter().map(|m| m.cost()).sum()
}

fn main() {
    let material1 = Carpet { amount: 15.0 };
    let material2 = Tile { amount: 35.0 };
    let material3 = Wood { amount: 12.0 };
    let materials: Vec<Box<dyn Material>> = vec![
        Box::new(material1),
        Box::new(material2),
        Box::new(material3),
    ];
    let cost = calculate_cost(&materials);
    println!("cost of the materials is {cost}$");
}
