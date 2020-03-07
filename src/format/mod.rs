mod hex;
mod plain1;
mod raw;
mod signal;

use self::hex::HexFormat;
use plain1::Plain1Format;
use raw::RawFormat;
use signal::SignalFormat;

use std::str::FromStr;

const PLAIN1_NAME: &str = "plain1";
const HEX_NAME: &str = "hex";
const SIGNAL_NAME: &str = "signal";
const RAW_NAME: &str = "raw";

pub const DEFAULT_KEY_FORMAT: &str = HEX_NAME;
pub const DEFAULT_PLAIN_FORMAT: &str = PLAIN1_NAME;
pub const DEFAULT_CIPHER_FORMAT: &str = SIGNAL_NAME;

pub static ALL_FORMAT_NAMES: &'static [&'static str] =
    &[PLAIN1_NAME, HEX_NAME, SIGNAL_NAME, RAW_NAME];

// Key format names does not include PLAIN1 because it has meaningful whitespace
// and is therefore not a robust way of passing keys around.
pub static KEY_FORMAT_NAMES: &'static [&'static str] = &[HEX_NAME, SIGNAL_NAME, RAW_NAME];

pub enum FormatType {
    Raw,
    Plain1,
    Hex,
    Signal,
}

impl FromStr for FormatType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            RAW_NAME => Ok(FormatType::Raw),
            PLAIN1_NAME => Ok(FormatType::Plain1),
            HEX_NAME => Ok(FormatType::Hex),
            SIGNAL_NAME => Ok(FormatType::Signal),
            _ => Err("no match"),
        }
    }
}

#[derive(Debug)]
pub enum FormatError {
    MalformedInput,
}

pub trait Format {
    fn pack(&self, input: &[u8]) -> Vec<u8>;
    fn unpack(&self, input: &[u8]) -> Result<Vec<u8>, FormatError>;
}

pub fn select_format(name: FormatType) -> Result<Box<dyn Format>, FormatError> {
    match name {
        FormatType::Raw => Ok(Box::new(RawFormat {})),
        FormatType::Plain1 => Ok(Box::new(Plain1Format {})),
        FormatType::Hex => Ok(Box::new(HexFormat {})),
        FormatType::Signal => Ok(Box::new(SignalFormat {})),
    }
}
