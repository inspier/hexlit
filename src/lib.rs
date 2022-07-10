//! This crate provides the `hex!` macro for converting
//! hexadecimal string literals to a byte array at compile
//! time.
//!
//! # Examples
//! ```
//! use hexlit::hex;
//!
//! const DATA: [u8; 4] = hex!("01020304");
//! assert_eq!(DATA, [1, 2, 3, 4]);
//! assert_eq!(hex!("0xDEADBEEF"), [0xDE, 0xAD, 0xBE, 0xEF]);
//! assert_eq!(hex!("a1b2c3d4"), [0xA1, 0xB2, 0xC3, 0xD4]);
//! assert_eq!(hex!("E5 E6 90 92"), [0xE5, 0xE6, 0x90, 0x92]);
//! assert_eq!(hex!("0a0B0C0d"), [10, 11, 12, 13]);
//! assert_eq!(hex!(0a "01" 0C 02), [10, 1, 12, 2]);
//! ```
#![no_std]

#[doc(hidden)]
#[macro_export]
macro_rules! require_even_number_digits {
    ($e:expr) => {
        let _: $crate::internals::Even<[(); $e % 2]>;
    };
}

#[macro_export]
macro_rules! hex {
    (@string $arg:expr) => {{
        const DATA: &[u8] = $arg.as_bytes();
        const RAW_LENGTH: usize = $arg.len() - $crate::internals::count_skipped(&DATA);
        $crate::require_even_number_digits!(RAW_LENGTH);
        $crate::internals::convert::<{RAW_LENGTH / 2}, {$arg.len()}>(&DATA)
    }};
    ($($tt:tt)*) => {
        hex!(@string stringify!($($tt)*))
    };
}

#[doc(hidden)]
pub mod internals {

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

    // Count the number of occurrences of a char.
    pub const fn count_skipped(data: &[u8]) -> usize {
        let mut char_count: usize = 0;
        let mut char_index: usize = 0;

        while char_index < data.len() {
            if char_index + 1 < data.len() && !is_valid_delimiter(data[char_index]) {
                let mut next_index = char_index + 1;
                while next_index < data.len() && is_valid_delimiter(data[next_index]) {
                    char_count += 1;
                    next_index += 1;
                }

                if data[char_index] == b'0'
                    && (data[next_index] == b'x' || data[next_index] == b'X')
                {
                    char_count += 2;
                }

                char_index = next_index + 1;
            } else {
                char_index += 1;
                char_count += 1;
            }
        }

        char_count
    }

    // Checks if part of set of valid delimiters.
    pub const fn is_valid_delimiter(c: u8) -> bool {
        matches!(c, b' ' | b'"' | b'_' | b'|' | b'-' | b'\n')
    }

    // Converts a individual byte into its correct integer
    // counter-part.
    #[allow(clippy::unnecessary_operation)]
    pub const fn to_ordinal(input: u8) -> u8 {
        match input {
            b'0'..=b'9' => input - b'0',
            b'A'..=b'F' => input - b'A' + 10,
            b'a'..=b'f' => input - b'a' + 10,
            _ => {
                #[allow(unconditional_panic)]
                ["Invalid hex digit."][({ true } as usize)];
                loop {} // Unreachable
            }
        }
    }

    // Converts a hex-string to its byte array representation.
    pub const fn convert<const RESULT_SIZE: usize, const STRING_SIZE: usize>(
        input: &[u8],
    ) -> [u8; RESULT_SIZE] {
        let mut data = [0_u8; RESULT_SIZE];
        let mut data_index: usize = 0;
        let mut char_index: usize = 0;

        while data_index < STRING_SIZE && char_index + 1 < STRING_SIZE {
            if !is_valid_delimiter(input[char_index]) {
                let mut next_index = char_index + 1;
                while next_index < STRING_SIZE && is_valid_delimiter(input[next_index]) {
                    next_index += 1;
                }

                if !(input[char_index] == b'0'
                    && (input[next_index] == b'x' || input[next_index] == b'X'))
                {
                    data[data_index] = to_ordinal(input[char_index]) * 16
                        + to_ordinal(input[next_index]);
                    data_index += 1;
                }
                char_index = next_index + 1;
            } else {
                char_index += 1;
            }
        }
        data
    }
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
    fn test_pipes() {
        assert_eq!(hex!(0A|0B|0C|0d), [10, 11, 12, 13]);
        assert_eq!(hex!(0F 03|0B|0C|0d), [15, 3, 11, 12, 13]);
    }

    #[test]
    fn test_dashes() {
        assert_eq!(hex!(0A-0B-0C-0d), [10, 11, 12, 13]);
        assert_eq!(hex!("0F 03-0B 0C-0d 0E"), [15, 3, 11, 12, 13, 14]);
    }

    #[test]
    fn test_mixed_no_quotes() {
        assert_eq!(hex!(1a 0b_0C 0d), [0x1a, 11, 12, 13]);
        assert_eq!(hex!(1a 0_b 0C 0d), [0x1a, 11, 12, 13]);
    }

    #[test]
    fn test_new_lines() {
        let a = hex!(
            00000000 00000000 00000000
            00000000
            00000000 00000000 00000000 00000000 00000000
        );
        let b = hex!(
            00000000 00000000 00000000 00000000
            00000000 00000000 00000000 00000000 00000000
        );
        let c = hex!(
            00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000
        );
        let d = hex!(00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000);

        assert_eq!(a, b);
        assert_eq!(b, c);
        assert_eq!(c, d);
    }

    #[test]
    fn test_hex_prefix() {
        assert_eq!(
            hex!("0xe9e7cea3dedca5984780bafc599bd69add087d56"),
            [
                0xE9, 0xE7, 0xCE, 0xA3, 0xDE, 0xDC, 0xA5, 0x98, 0x47, 0x80, 0xBA, 0xFC,
                0x59, 0x9B, 0xD6, 0x9A, 0xDD, 0x08, 0x7D, 0x56
            ]
        );
    }
}
