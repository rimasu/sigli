use crate::format::{Format, FormatError};

pub struct RawFormat {}

pub const FORMAT_NAME: &str = "raw";

impl Format for RawFormat {
    fn unpack_input(&self, _output: &mut Vec<u8>) -> Result<(), FormatError> {
        // Nothing to do
        Ok(())
    }

    fn pack_output(&self, _output: &mut Vec<u8>) {
        // Nothing to do
    }
}
