mod core;
mod format;

use crate::core::select_algorithm;
use crate::format::select_format;

pub use crate::core::{AlgoType, CoreError, ALGORITHM_NAMES, DEFAULT_ALGO_NAME};

pub use crate::format::{FormatType, FormatError,
                        ALL_FORMAT_NAMES,
                        KEY_FORMAT_NAMES,
                        DEFAULT_KEY_FORMAT,
                        DEFAULT_PLAIN_FORMAT,
                        DEFAULT_CIPHER_FORMAT, };

#[derive(Debug)]
pub enum SigliError {
    Core(CoreError),
    Format(FormatError),
}

impl std::convert::From<CoreError> for SigliError {
    fn from(e: CoreError) -> Self {
        SigliError::Core(e)
    }
}

impl std::convert::From<FormatError> for SigliError {
    fn from(e: FormatError) -> Self {
        SigliError::Format(e)
    }
}

pub fn generate_key(
    algo_type: AlgoType,
    key_format: FormatType,
) -> Result<Vec<u8>, SigliError>
{
    let key = select_algorithm(algo_type)
        .map(|a| a.generate_key_data())?;

    let raw_key = select_format(key_format)
        .map(|f| f.pack(&key))?;

    Ok(raw_key)
}


pub fn encrypt(
    algo_type: AlgoType,
    key_format: FormatType,
    input_format: FormatType,
    output_format: FormatType,
    raw_key: &[u8],
    raw_input: &[u8],
) -> Result<Vec<u8>, SigliError>
{
    let key = select_format(key_format)
        .and_then(|f| f.unpack(raw_key))?;

    let input = select_format(input_format)
        .and_then(|f| f.unpack(raw_input))?;

    let output = select_algorithm(algo_type)
        .and_then(|a| a.encrypt_data(&key, &input))?;

    let raw_output = select_format(output_format)
        .map(|f| f.pack(&output))?;

    Ok(raw_output)
}

pub fn decrypt(
    algo_type: AlgoType,
    key_format: FormatType,
    input_format: FormatType,
    output_format: FormatType,
    raw_key: &[u8],
    raw_input: &[u8],
) -> Result<Vec<u8>, SigliError>
{
    let key = select_format(key_format)
        .and_then(|f| f.unpack(raw_key))?;

    let input = select_format(input_format)
        .and_then(|f| f.unpack(raw_input))?;

    let output = select_algorithm(algo_type)
        .and_then(|a| a.decrypt_data(&key, &input))?;

    let raw_output = select_format(output_format)
        .map(|f| f.pack(&output))?;

    Ok(raw_output)
}
