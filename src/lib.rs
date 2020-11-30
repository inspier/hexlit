//! This crate provides the `hex!` macro for converting hexadecimal string literals
//! to a byte array at compile time.
//!
//! # Examples
//! ```
//! use hexlit::hex;
//!
//! fn main() {
//! const DATA: [u8; 4] = hex!("01020304");
//! assert_eq!(DATA, [1, 2, 3, 4]);
//! assert_eq!(hex!("a1b2c3d4"), [0xA1, 0xB2, 0xC3, 0xD4]);
//! assert_eq!(hex!("E5 E6 90 92"), [0xE5, 0xE6, 0x90, 0x92]);
//! assert_eq!(hex!("0a0B0C0d"), [10, 11, 12, 13]);
//! assert_eq!(hex!(0a "01" 0C 02), [10, 1, 12, 2]);
//! }
//! ```
#![no_std]

#[doc(hidden)]
#[macro_export]
macro_rules! require_even_number_digits {
    ($e:expr) => {
        let _: $crate::Even<[(); $e % 2]>;
    };
}

pub type Even<T> =
    <<T as HexStringLength>::Marker as LengthIsEvenNumberOfHexDigits>::Check;

pub enum IsEvenNumberofDigits {}
pub enum IsOddNumberofDigits {}

pub trait HexStringLength {
    type Marker;
}

impl HexStringLength for [(); 0] {
    type Marker = IsEvenNumberofDigits;
}

impl HexStringLength for [(); 1] {
    type Marker = IsOddNumberofDigits;
}

pub trait LengthIsEvenNumberOfHexDigits {
    type Check;
}

impl LengthIsEvenNumberOfHexDigits for IsEvenNumberofDigits {
    type Check = ();
}

#[macro_export]
macro_rules! hex {
    (@string $arg:expr) => {{
        const DATA: &[u8] = $arg.as_bytes();

        const fn count_occurrences(data: &[u8], c: u8) -> usize {
            let mut char_count: usize = 0;
            let mut char_index: usize = 0;
            while char_index < data.len() {
                if data[char_index] == c {
                    char_count += 1;
                }
                char_index += 1;
            }
            char_count
        }

        const NUM_SPACES: usize = count_occurrences(DATA, b' ');
        const NUM_UNDERSCORES: usize = count_occurrences(DATA, b'_');
        const NUM_QUOTES: usize = count_occurrences(DATA, b'"');

        const NUM_SKIPPED: usize = NUM_SPACES + NUM_UNDERSCORES + NUM_QUOTES;

        $crate::require_even_number_digits!($arg.len() - NUM_SKIPPED);
        const ARRAY_LENGTH: usize = ($arg.len() - NUM_SKIPPED) / 2;
        const RESULT: [u8; ARRAY_LENGTH] = {
            // Hack needed for const-eval to work.
            const fn always_true() -> bool {
                true
            }

            /// Converts a individual byte into its correct integer counter-part.
            const fn to_ordinal(input: u8) -> u8 {
                match input {
                    b'0'..=b'9' => input - b'0',
                    b'A'..=b'F' => input - b'A' + 10,
                    b'a'..=b'f' => input - b'a' + 10,
                    _ => {
                        ["Invalid hex digit."][(always_true() as usize)];
                        0 // Unreachable
                    }
                }
            }

            // Converts a hex-string to its byte array representation.
                let mut data = [0u8; ARRAY_LENGTH];
                let mut data_index: usize = 0;
                let mut char_index: usize = 0;
                let string_length = $arg.len();
                while data_index < string_length && char_index + 1 < string_length {
                    if DATA[char_index] != b' ' && DATA[char_index] != b'_' && DATA[char_index] != b'"' {
                        let mut next_index = char_index + 1;
                        while next_index < string_length && (DATA[next_index] == b' ' || DATA[next_index] == b'_' || DATA[next_index] == b'"') {
                            next_index += 1;
                        }
                        data[data_index] = to_ordinal(DATA[char_index]) * 16 + to_ordinal(DATA[next_index]);
                        char_index = next_index + 1;
                        data_index += 1;
                    } else {
                        char_index += 1;
                    }
                }
                data
        };
        RESULT
    }};
    ($($tt:tt)*) => {
        hex!(@string stringify!($($tt)*))
    };
}

#[cfg(test)]
mod tests {
    use super::hex;

    #[test]
    fn test_leading_zeros() {
        assert_eq!(hex!("01020304"), [1, 2, 3, 4]);
    }

    #[test]
    fn test_alphanumeric_lower() {
        assert_eq!(hex!("a1b2c3d4"), [0xA1, 0xB2, 0xC3, 0xD4]);
    }

    #[test]
    fn test_alphanumeric_upper() {
        assert_eq!(hex!("E5E69092"), [0xE5, 0xE6, 0x90, 0x92]);
    }

    #[test]
    fn test_alphanumeric_mixed() {
        assert_eq!(hex!("0a0B0C0d"), [10, 11, 12, 13]);
    }

    #[test]
    fn test_leading_zeros_space() {
        assert_eq!(hex!("01 02 03 04"), [1, 2, 3, 4]);
    }

    #[test]
    fn test_alphanumeric_lower_space() {
        assert_eq!(hex!("a1 b2 c3 d4"), [0xA1, 0xB2, 0xC3, 0xD4]);
    }

    #[test]
    fn test_alphanumeric_upper_space() {
        assert_eq!(hex!("E5 E6 90 92"), [0xE5, 0xE6, 0x90, 0x92]);
    }

    #[test]
    fn test_alphanumeric_mixed_space() {
        assert_eq!(hex!("0a 0B 0C 0d"), [10, 11, 12, 13]);
    }

    #[test]
    fn test_no_quotes() {
        assert_eq!(hex!(a0 0B 0C 0d), [0xa0, 11, 12, 13]);
    }

    #[test]
    fn test_weird_quotes() {
        assert_eq!(hex!(a0 "0b" 0C 0d), [0xa0, 11, 12, 13]);
    }

    #[test]
    fn test_no_quotes_start_with_zero() {
        assert_eq!(hex!(0A 0B 0C0d), [10, 11, 12, 13]);
    }

    #[test]
    fn test_underscores() {
        assert_eq!(hex!(0A_0B_0C 0d), [10, 11, 12, 13]);
    }

    #[test]
    fn test_mixed_no_quotes() {
        assert_eq!(hex!(1a 0b_0C 0d), [0x1a, 11, 12, 13]);
        assert_eq!(hex!(1a 0_b 0C 0d), [0x1a, 11, 12, 13]);
    }
}
