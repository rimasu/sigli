mod hex;
mod plain1;
mod raw;
mod signal;

use std::str::FromStr;

pub const DEFAULT_KEY_FORMAT: &str = self::hex::FORMAT_NAME;
pub const DEFAULT_PLAIN_FORMAT: &str = self::plain1::FORMAT_NAME;
pub const DEFAULT_CIPHER_FORMAT: &str = self::signal::FORMAT_NAME;

pub static ALL_FORMAT_NAMES: &[&str] = &[
    self::plain1::FORMAT_NAME,
    self::hex::FORMAT_NAME,
    self::signal::FORMAT_NAME,
    self::raw::FORMAT_NAME,
];

// Key format names does not include PLAIN1 because it has meaningful whitespace
// and is therefore not a robust way of passing keys around.
pub static KEY_FORMAT_NAMES: &[&str] = &[
    self::plain1::FORMAT_NAME,
    self::hex::FORMAT_NAME,
    self::signal::FORMAT_NAME,
    self::raw::FORMAT_NAME,
];

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
            self::raw::FORMAT_NAME => Ok(FormatType::Raw),
            self::plain1::FORMAT_NAME => Ok(FormatType::Plain1),
            self::hex::FORMAT_NAME => Ok(FormatType::Hex),
            self::signal::FORMAT_NAME => Ok(FormatType::Signal),
            _ => Err("no match"),
        }
    }
}

#[derive(Debug)]
pub enum FormatError {
    MalformedInput,
}

pub trait Format {
    fn pack(&self, output: &mut Vec<u8>);
    fn unpack(&self, input: &[u8]) -> Result<Vec<u8>, FormatError>;
}

pub fn select_format(name: FormatType) -> Result<Box<dyn Format>, FormatError> {
    match name {
        FormatType::Raw => Ok(Box::new(self::raw::RawFormat {})),
        FormatType::Plain1 => Ok(Box::new(self::plain1::Plain1Format {})),
        FormatType::Hex => Ok(Box::new(self::hex::HexFormat {})),
        FormatType::Signal => Ok(Box::new(self::signal::SignalFormat {})),
    }
}
