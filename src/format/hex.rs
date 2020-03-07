use super::{Format, FormatError};

pub struct HexFormat {}

impl Format for HexFormat {
    fn pack(&self, input: &[u8]) -> Vec<u8> {
        let mut key_text = String::new();

        for (idx, c) in hex::encode_upper(input).chars().enumerate() {
            key_text.push(c);
            if idx % 4 == 3 {
                key_text.push('-')
            }
        }

        if key_text.chars().last() == Some('-') {
            key_text.remove(key_text.len() - 1);
        }

        key_text.push('\n');

        key_text.as_bytes().to_vec()
    }

    fn unpack(&self, input: &[u8]) -> Result<Vec<u8>, FormatError> {
        let text = std::str::from_utf8(input)
            .map_err(|_| FormatError::MalformedInput)?;

        let mut clean_text = String::with_capacity(text.len());
        for c in text.chars() {
            if c.is_ascii_hexdigit() {
                clean_text.push(c)
            } else if c != '-' && !c.is_whitespace() {
                return Err(FormatError::MalformedInput);
            }
        }


        hex::decode(clean_text)
            .map_err(|_| FormatError::MalformedInput)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_pack_hex_input() {
        let raw = vec![0xAB, 0x01, 0x02, 0x22, 0x23, 0x43];
        let packed = HexFormat {}.pack(&raw);
        assert_eq!("AB01-0222-2343\n".as_bytes().to_vec(), packed);
    }

    #[test]
    fn can_unpack_hex_input_with_hyphens() {
        let raw = "AB01-0222-2343\n".as_bytes();
        let unpacked = HexFormat {}.unpack(raw).unwrap();
        assert_eq!(vec![0xAB, 0x01, 0x02, 0x22, 0x23, 0x43], unpacked);
    }

    #[test]
    fn can_unpack_hex_input_without_hyphens() {
        let raw = "AB0102222343".as_bytes();
        let unpacked = HexFormat {}.unpack(raw).unwrap();
        assert_eq!(vec![0xAB, 0x01, 0x02, 0x22, 0x23, 0x43], unpacked);
    }

    #[test]
    fn can_unpack_lower_case_hex_input_without_hyphens() {
        let raw = "ab0102222343".as_bytes();
        let unpacked = HexFormat {}.unpack(raw).unwrap();
        assert_eq!(vec![0xAB, 0x01, 0x02, 0x22, 0x23, 0x43], unpacked);
    }

    #[test]
    fn can_unpack_hex_input_with_spaces() {
        let raw = "AB0 1 022 2 2343  ".as_bytes();
        let unpacked = HexFormat {}.unpack(raw).unwrap();
        assert_eq!(vec![0xAB, 0x01, 0x02, 0x22, 0x23, 0x43], unpacked);
    }
}