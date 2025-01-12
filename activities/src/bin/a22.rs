// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n <= lower {
        lower
    } else if n >= upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        return None;
    }
    Some(a / b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(5, 1, 3), 3);
        assert_eq!(clamp(1, 4, 5), 4);
        assert_eq!(clamp(5, 1, 10), 5);
    }

    #[test]
    fn test_div() {
        assert_eq!(div(10.0, 2.0), Some(5.0));
        assert_eq!(div(5.0, 0.0), None);
        assert_eq!(div(9.0, 3.0), Some(3.0));
    }

    #[test]
    fn test_concat() {
        assert_eq!(concat("hey", "you"), "heyyou");
        assert_eq!(concat("two", "three"), "twothree");
    }
}
