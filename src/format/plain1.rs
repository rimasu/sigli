use super::{Format, FormatError};
use convert_base::Convert;
use std::collections::HashMap;

pub struct Plain1Format {}

pub const FORMAT_NAME: &str = "plain1";

static PLAIN_ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz09123456789 ,.";

impl Format for Plain1Format {
    fn pack(&self, output: &mut Vec<u8>) {
        let mut lookup = HashMap::new();
        for (idx, c) in PLAIN_ALPHABET.chars().enumerate() {
            lookup.insert(idx as u8, c as u32 as u8);
        }

        let mut convert = Convert::new(256, PLAIN_ALPHABET.len() as u64);
        let buf = convert.convert::<u8, u8>(output);
        output.clear();
        for point in buf {
            output.push(*lookup.get(&point).unwrap())
        }
    }

    fn unpack(&self, input: &[u8]) -> Result<Vec<u8>, FormatError> {
        let input = std::str::from_utf8(input).map_err(|_| FormatError::MalformedInput)?;

        let mut lookup = HashMap::new();
        for (idx, c) in PLAIN_ALPHABET.chars().enumerate() {
            lookup.insert(c, idx as u8);
        }

        let mut buf = Vec::with_capacity(input.len());

        for mut char in input.chars() {
            if char != '\r' {
                if char.is_whitespace() {
                    char = ' ';
                }
                char.make_ascii_lowercase();
                if let Some(idx) = lookup.get(&char) {
                    buf.push(*idx)
                }
            }
        }

        let mut convert = Convert::new(PLAIN_ALPHABET.len() as u64, 256);
        Ok(convert.convert::<u8, u8>(&buf))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn all_bytes_unpacked() -> Vec<u8> {
        vec![
            168, 186, 91, 207, 48, 190, 246, 166, 32, 30, 39, 12, 102, 40, 77, 101, 112, 232, 61,
            55, 138, 173, 223, 183, 55, 94, 29
        ]
    }

    fn all_letters_packed() -> Vec<u8> {
        "abcdefghijklmnopqrstuvwxyz09123456789 ,."
            .as_bytes()
            .to_vec()
    }

    #[test]
    fn can_pack_any_byte() {
        let mut output = all_bytes_unpacked();
        Plain1Format {}.pack(&mut raw);
        assert_eq!(all_letters_packed(), output);
    }

    #[test]
    fn can_unpack_any_byte_with_lowercase() {
        let raw = all_letters_packed();
        let unpacked = Plain1Format {}.unpack(&raw).unwrap();
        assert_eq!(all_bytes_unpacked(), unpacked)
    }

    #[test]
    fn can_unpack_any_byte_with_uppercase() {
        let raw = "abcdefghijklmnopqrstuvwxyz09123456789 ,."
            .to_uppercase()
            .as_bytes()
            .to_vec();

        let unpacked = Plain1Format {}.unpack(&raw).unwrap();
        assert_eq!(all_bytes_unpacked(), unpacked)
    }

    #[test]
    fn can_unpack_any_byte_with_newline() {
        let raw = "abcdefghijklmnopqrstuvwxyz09123456789\n,."
            .to_uppercase()
            .as_bytes()
            .to_vec();

        let unpacked = Plain1Format {}.unpack(&raw).unwrap();
        assert_eq!(all_bytes_unpacked(), unpacked)
    }

    #[test]
    fn can_unpack_any_byte_with_carriage_return_and_newline() {
        let raw = "abcdefghijklmnopqrstuvwxyz09123456789\r\n,."
            .to_uppercase()
            .as_bytes()
            .to_vec();

        let unpacked = Plain1Format {}.unpack(&raw).unwrap();
        assert_eq!(all_bytes_unpacked(), unpacked)
    }
}
