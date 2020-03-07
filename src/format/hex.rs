use super::{Format, FormatError};

pub struct HexFormat {}

pub const FORMAT_NAME: &str = "hex";

impl Format for HexFormat {
    fn pack(&self, output: &mut Vec<u8>) {
        let buf = hex::encode_upper(&output);
        output.clear();

        for (idx, c) in buf.chars().enumerate() {
            output.push(c as u32 as u8);
            if idx % 4 == 3 {
                output.push(45)
            }
        }

        if output.ends_with(&[45]) {
            output.remove(output.len() - 1);
        }

        output.push(10);
    }

    fn unpack(&self, input: &[u8]) -> Result<Vec<u8>, FormatError> {
        let text = std::str::from_utf8(input).map_err(|_| FormatError::MalformedInput)?;

        let mut clean_text = String::with_capacity(text.len());
        for c in text.chars() {
            if c.is_ascii_hexdigit() {
                clean_text.push(c)
            } else if c != '-' && !c.is_whitespace() {
                return Err(FormatError::MalformedInput);
            }
        }

        hex::decode(clean_text).map_err(|_| FormatError::MalformedInput)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_pack_hex_input() {
        let mut output  = vec![0xAB, 0x01, 0x02, 0x22, 0x23, 0x43];
        HexFormat {}.pack(&mut &output);
        assert_eq!("AB01-0222-2343\n".as_bytes().to_vec(), output);
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
