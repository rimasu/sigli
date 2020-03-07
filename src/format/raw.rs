use crate::format::{Format, FormatError};

pub struct RawFormat {}

pub const FORMAT_NAME: &str = "raw";

impl Format for RawFormat {
    fn pack(&self, _output: &mut Vec<u8>) {
        // Nothing to do
    }

    fn unpack(&self, input: &[u8]) -> Result<Vec<u8>, FormatError> {
        let mut result = Vec::with_capacity(input.len());
        result.extend_from_slice(input);
        Ok(result)
    }
}
