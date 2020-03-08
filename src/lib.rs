mod algo;
mod format;

use crate::algo::select_algorithm;
use crate::format::select_format;

pub use crate::algo::{AlgoError, AlgoType, ALGORITHM_NAMES, DEFAULT_ALGO_NAME};

pub use crate::format::{
    FormatError, FormatType, ALL_FORMAT_NAMES, DEFAULT_CIPHER_FORMAT, DEFAULT_KEY_FORMAT,
    DEFAULT_PLAIN_FORMAT, KEY_FORMAT_NAMES,
};

#[derive(Debug)]
pub enum SigliError {
    Algo(AlgoError),
    Format(FormatError),
}

impl std::convert::From<AlgoError> for SigliError {
    fn from(e: AlgoError) -> Self {
        SigliError::Algo(e)
    }
}

impl std::convert::From<FormatError> for SigliError {
    fn from(e: FormatError) -> Self {
        SigliError::Format(e)
    }
}

pub fn generate_key(algo_type: AlgoType, key_format: FormatType) -> Result<Vec<u8>, SigliError> {
    let mut key = select_algorithm(algo_type).map(|a| a.generate_key_data())?;

    select_format(key_format).map(|f| f.pack(&mut key))?;

    Ok(key)
}

pub fn encrypt(
    algo_type: AlgoType,
    key_format: FormatType,
    input_format: FormatType,
    output_format: FormatType,
    raw_key: &[u8],
    raw_input: &[u8],
) -> Result<Vec<u8>, SigliError> {
    let key = select_format(key_format).and_then(|f| f.unpack(raw_key))?;

    let mut data = select_format(input_format).and_then(|f| f.unpack(raw_input))?;

    select_algorithm(algo_type).and_then(|a| a.encrypt_data(&key, &mut data))?;

    select_format(output_format).map(|f| f.pack(&mut data))?;

    Ok(data)
}

pub fn decrypt(
    algo_type: AlgoType,
    key_format: FormatType,
    input_format: FormatType,
    output_format: FormatType,
    raw_key: &[u8],
    raw_input: &[u8],
) -> Result<Vec<u8>, SigliError> {
    let key = select_format(key_format).and_then(|f| f.unpack(raw_key))?;

    let mut data = select_format(input_format).and_then(|f| f.unpack(raw_input))?;

    select_algorithm(algo_type).and_then(|a| a.decrypt_data(&key, &mut data))?;

    select_format(output_format).map(|f| f.pack(&mut data))?;

    Ok(data)
}

