use crate::format::{Format, FormatError};

pub struct RawFormat {}

impl Format for RawFormat {
    fn pack(&self, input: &[u8]) -> Vec<u8> {
        let mut result = Vec::with_capacity(input.len());
        result.extend_from_slice(input);
        result
    }

    fn unpack(&self, input: &[u8]) -> Result<Vec<u8>, FormatError> {
        let mut result = Vec::with_capacity(input.len());
        result.extend_from_slice(input);
        Ok(result)
    }
}
