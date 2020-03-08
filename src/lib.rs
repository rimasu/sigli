//! Lib Sigli simple cipher wrapper to support Sigli CLI.
//!
//! Library provides encrypt/decrypt functionality wrapped with simple input and output
//! formatting.
//!
//! # Example
//!
//! ```rust
//! # use std::error::Error;
//! # fn main() -> Result<(), Box<dyn Error>> {
//! use sigli::{AlgoType, FormatType, generate_key, encrypt, decrypt};
//!
//! let key = generate_key(AlgoType::Aes256Gcm, FormatType::Hex)?;
//!
//! let input_plain_text = "test message 12. ".as_bytes().to_vec();
//!
//! // Copy input text because encrypt/decrypt operate 'in place'.
//! let mut data = input_plain_text.clone();
//!
//! encrypt(
//!      AlgoType::Aes256Gcm, // Encryption algorithm
//!     FormatType::Hex,      // Format of key
//!     FormatType::Plain1,   // Format of input (plain text)
//!     FormatType::Signal1,  // Format of output (cipher text)
//!     &mut key.clone(),
//!     &mut data
//! )?;
//!
//! println!("{}", std::str::from_utf8(&data)?);
//!
//! // Will print cipher text something like:
//! //
//! // ZKCNU ZOSJI INMQH YBFNP BKBSY XGZWK
//! // PMXVZ DLRDK TPBCQ EFIYS ZRHPS XUEJL
//! // JKKBG YRN
//! //
//! // but exact content will vary randomly.
//!
//! decrypt(
//!     AlgoType::Aes256Gcm, // Decryption algorithm
//!     FormatType::Hex,     // Format of key
//!     FormatType::Signal1, // Format of input (cipher text)
//!     FormatType::Plain1,  // Format of output (plain text)
//!     &mut key.clone(),
//!     &mut data
//! )?;
//!
//! assert_eq!(&input_plain_text.to_vec(),  &data);
//! # Ok(())
//! # }


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
    MalformedKey(FormatError),
    MalformedInput(FormatError)
}

impl std::fmt::Display for SigliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self)
    }
}

impl std::error::Error for SigliError {}

impl std::convert::From<AlgoError> for SigliError {
    fn from(e: AlgoError) -> Self {
        SigliError::Algo(e)
    }
}



/// Generate a new key.
///
/// # Arguments
///
/// * `algo` - Algorithm to use for decryption.
/// * `key_format` - Format used to pack key.
///
/// # Example
///
/// ```rust
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
///
/// use sigli::{AlgoType, FormatType, generate_key};
///
/// let key = generate_key(
///     AlgoType::Aes256Gcm, // Algorithm to generate key for
///     FormatType::Hex,     // Format of key
/// )?;
///
/// // Print out key.
/// println!("{:?}", std::str::from_utf8(&key)?);
///
/// # Ok(())
/// # }
///
/// ```
pub fn generate_key(algo_type: AlgoType, key_format: FormatType) -> Result<Vec<u8>, SigliError> {
    let mut key = select_algorithm(algo_type).generate_key_data();

    select_format(key_format).pack_output(&mut key);

    Ok(key)
}


/// Encrypt message data in place.
///
/// # Arguments
///
/// * `algorithm` - Algorithm to use for decryption
/// * `key_format` - Format used to unpack raw key into key data.
/// * `input_format` - Format used to unpack raw input data into input data.
/// * `output_format` - Format used to pack output data input raw output data.
/// * `key` - Raw key data. On successful return this will have been converted into key data.
/// * `data` - Raw data to decrypt. On successful return this will have been converted into
/// raw output data.
///
/// # Example
///
/// ```rust
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// use sigli::{AlgoType, FormatType, encrypt};
///
/// // AES-256-GCM Key is 256 bits long (32 bytes)
/// let mut key = "E1EB-4267-D828-2ADB-FF47-\
///                E431-ABAF-FC2D-84E7-E045-\
///                9CEE-2C39-487D-A576-ECF4-\
///                FD53".as_bytes().to_vec();
///
/// // Plain text in 'plain1' format. This can have ascii letters, numbers spaces and full
/// // stop (period).
/// let mut data = "test message 12. ".as_bytes().to_vec();
///
/// encrypt(
///     AlgoType::Aes256Gcm, // Decryption algorithm
///     FormatType::Hex,     // Format of key
///     FormatType::Plain1,  // Format of input (plain text)
///     FormatType::Signal1,  // Format of output (cipher text)
///     &mut key,
///     &mut data
/// )?;
///
/// // Print out cipher text. In this example we are using a randomly generated nonce, so
/// // the output will vary on each run.
/// println!("{:?}", std::str::from_utf8(&data)?);
///
/// # Ok(())
/// # }
/// ```
pub fn encrypt(
    algorithm: AlgoType,
    key_format: FormatType,
    input_format: FormatType,
    output_format: FormatType,
    key: &mut Vec<u8>,
    data: &mut Vec<u8>,
) -> Result<(), SigliError> {

    select_format(key_format)
        .unpack_input(key)
        .map_err(|e| SigliError::MalformedKey(e))?;

    select_format(input_format)
        .unpack_input(data)
        .map_err(|e| SigliError::MalformedInput(e))?;

    select_algorithm(algorithm).encrypt_data(key, data)?;
    select_format(output_format).pack_output(data);

    Ok(())
}


/// Decrypt message data in place.
///
/// # Arguments
///
/// * `algorithm` - Algorithm to use for decryption
/// * `key_format` - Format used to unpack raw key into key data.
/// * `input_format` - Format used to unpack raw input data into input data.
/// * `output_format` - Format used to pack output data input raw output data.
/// * `key` - Raw key data. On successful return this will have been converted into key data.
/// * `data` - Raw data to decrypt. On successful return this will have been converted into
/// raw output data.
///
/// # Example
///
/// ```rust
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// use sigli::{AlgoType, FormatType, decrypt};
///
/// // AES-256-GCM Key is 256 bits long (32 bytes)
/// let mut key = "E1EB-4267-D828-2ADB-FF47-\
///                E431-ABAF-FC2D-84E7-E045-\
///                9CEE-2C39-487D-A576-ECF4-\
///                FD53".as_bytes().to_vec();
///
/// // Cipher data packed using the 'signal' format that breaks
/// // it into groups of five ascii characters.
/// let mut data = "ZKCNU ZOSJI INMQH YBFNP BKBSY XGZWK\n\
///                 PMXVZ DLRDK TPBCQ EFIYS ZRHPS XUEJL\n\
///                 JKKBG YRN\n".as_bytes().to_vec();
///
/// decrypt(
///     AlgoType::Aes256Gcm, // Decryption algorithm
///     FormatType::Hex,     // Format of key
///     FormatType::Signal1,  // Format of input (cipher text)
///     FormatType::Plain1,  // Format of output (plain text)
///     &mut key,
///     &mut data
/// )?;
///
/// assert_eq!(&"test message 12. ".as_bytes().to_vec(), &data);
/// # Ok(())
/// # }
///
/// ```
pub fn decrypt(
    algorithm: AlgoType,
    key_format: FormatType,
    input_format: FormatType,
    output_format: FormatType,
    key: &mut Vec<u8>,
    data: &mut Vec<u8>,
) -> Result<(), SigliError> {

    select_format(key_format)
        .unpack_input(key)
        .map_err(|e| SigliError::MalformedKey(e))?;

    select_format(input_format)
        .unpack_input(data)
        .map_err(|e| SigliError::MalformedInput(e))?;

    select_algorithm(algorithm).decrypt_data(key, data)?;

    select_format(output_format).pack_output(data);

    Ok(())
}
