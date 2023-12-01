// Topic: TryFrom/TryInto
//
// Summary:
// * A library is needed for an application to convert hex color codes
//   into their component color values (red, green, and blue). Hex color codes
//   consist of a hash symbol followed by six hex digits. Every two hex digits
//   represent a color component in the order of red, green, blue.
//
//   Example hex color codes:
//    #ffffff -> Rgb(255, 255, 255)
//    #001122 -> Rgb(0, 17, 34)
//
// Requirements:
// * Create a program to convert a hex code (as &str) into an Rgb struct
// * Implement TryFrom to perform the conversion
// * Utilize the question mark operator in your implementation
//
// Notes:
// * See the `from_str_radix` function in the stdlib docs for `u8`
//   to convert hex digits to `u8`
//   * Hex digits use a radix value of 16
// * Utilize the `thiserror` crate for your error type
// * Run `cargo test --bin a37` to test your implementation

use std::convert::TryFrom;

use thiserror::Error;

#[derive(Debug, Eq, PartialEq)]
struct Rgb(u8, u8, u8);

#[derive(Debug, Error)]
enum ConversionError {
    #[error("The value is not a proper hex color string.")]
    NotAHex,
    #[error("The value does not have exactly 3 values.")]
    Not3Values,
}

impl TryFrom<&str> for Rgb {
    type Error = ConversionError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if !value.starts_with('#') || value.len() != 7 {
            return Err(ConversionError::NotAHex);
        }
        let values = value[1..]
            .chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .map(|pair| pair.iter().collect::<String>())
            .map(|s| u8::from_str_radix(&s, 16).map_err(|_| ConversionError::NotAHex))
            .collect::<Result<Vec<u8>, ConversionError>>()?;
        if values.len() != 3 {
            return Err(ConversionError::Not3Values);
        }
        Ok(Rgb(values[0], values[1], values[2]))
    }
}

fn main() {
    // Use `cargo test --bin a37` to test your implementation
}

#[cfg(test)]
mod test {
    use super::Rgb;
    use std::convert::TryFrom;

    #[test]
    fn converts_valid_hex_color() {
        let expected = Rgb(0, 204, 102);
        let actual = Rgb::try_from("#00cc66");
        assert!(actual.is_ok(), "valid hex code should be converted to Rgb");
        assert_eq!(actual.unwrap(), expected, "wrong Rgb value");
    }

    #[test]
    fn fails_on_invalid_hex_digits() {
        assert!(
            Rgb::try_from("#0011yy").is_err(),
            "should be an error with invalid hex color"
        );
    }

    #[test]
    fn fails_when_missing_hash() {
        assert!(
            Rgb::try_from("001100").is_err(),
            "should be an error when missing hash symbol"
        );
    }

    #[test]
    fn fails_when_missing_color_components() {
        assert!(
            Rgb::try_from("#0011f").is_err(),
            "should be an error when missing one or more color components"
        );
    }

    #[test]
    fn fails_with_too_many_color_components() {
        assert!(
            Rgb::try_from("#0011ffa").is_err(),
            "should be an error when too many color components are provided"
        );
    }
}
