mod plain1;
mod hex;
mod signal;
mod raw;

use plain1::Plain1Format;
use self::hex::HexFormat;
use signal::SignalFormat;
use raw::RawFormat;

use std::str::FromStr;


pub const PLAIN1_NAME: &str = "plain1";
pub const HEX_NAME: &str = "hex";
pub const SIGNAL_NAME: &str = "signal";
pub const RAW_NAME: &str = "raw";

pub static FORMAT_NAMES: &'static [&'static str] = &[PLAIN1_NAME, HEX_NAME, SIGNAL_NAME, RAW_NAME];
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
    MalformedInput
}

pub trait Format {
    fn pack(&self, input: &[u8]) -> Result<Vec<u8>, FormatError>;
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