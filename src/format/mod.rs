mod hex;
mod plain1;
mod raw;
mod signal1;

use std::str::FromStr;

pub const DEFAULT_KEY_FORMAT: &str = self::hex::FORMAT_NAME;
pub const DEFAULT_PLAIN_FORMAT: &str = self::plain1::FORMAT_NAME;
pub const DEFAULT_CIPHER_FORMAT: &str = self::signal1::FORMAT_NAME;

pub static ALL_FORMAT_NAMES: &[&str] = &[
    self::plain1::FORMAT_NAME,
    self::hex::FORMAT_NAME,
    self::signal1::FORMAT_NAME,
    self::raw::FORMAT_NAME,
];

// Key format names does not include PLAIN1 because it has meaningful whitespace
// and is therefore not a robust way of passing keys around.
pub static KEY_FORMAT_NAMES: &[&str] = &[
    self::plain1::FORMAT_NAME,
    self::hex::FORMAT_NAME,
    self::signal1::FORMAT_NAME,
    self::raw::FORMAT_NAME,
];

/// Format used to either unpack inputs or pack outputs.
pub enum FormatType {
    /// Raw Binary
    ///
    /// Format is not changed. This is useful when 'plain text' is actually
    /// some binary format.
    Raw,

    /// Plait Text Format 1 (letters, numbers, space and period).
    ///
    /// Input format must consist entirely of
    /// 1. ASCII letters (will be lower cased)
    /// 2. ASCII numbers
    /// 3. A full stop (period)
    /// 4. Whitespace (will be converted to a ASCII 32 space character)
    /// # Example Value
    /// ```text
    /// party at edwards. 1234 hrs. bring drinks
    /// ```
    Plain1,

    /// Hexadecimal Format
    ///
    /// Input consisting of hexadecimal characters (upper or lower case).
    /// Any non-hex characters are ignored.
    /// Always generates upper case hex, grouped into blocks of 4 digits
    /// separated by hyphens.
    ///
    /// # Example Value
    /// ```text
    /// E1EB-4267-D828-2ADB-FF47-E431-ABAF-FC2D-84E7-E045
    /// ```
    Hex,

    /// Signal Format 1
    ///
    /// Input consisting of upper case ASCII letters.
    /// Some white space is ignored (line feed, carriage return and space).
    /// Always generates uppercase ASCII letters in blocks of five
    /// characters separated by spaces. A line feed is included
    /// after every sixth block.
    ///
    /// This is an expensive format to generate and should not be used
    /// for very long messages.
    ///
    /// # Example Value
    /// ```text
    /// ZKCNU ZOSJI INMQH YBFNP BKBSY XGZWK
    /// PMXVZ DLRDK TPBCQ EFIYS ZRHPS XUEJL
    /// JKKBG YRN
    /// ```
    Signal1,
}

impl FromStr for FormatType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            self::raw::FORMAT_NAME => Ok(FormatType::Raw),
            self::plain1::FORMAT_NAME => Ok(FormatType::Plain1),
            self::hex::FORMAT_NAME => Ok(FormatType::Hex),
            self::signal1::FORMAT_NAME => Ok(FormatType::Signal1),
            _ => Err("no match"),
        }
    }
}

#[derive(Debug)]
pub enum FormatError {
    MalformedInput,
}

pub trait Format {
    fn unpack_input(&self, input: &mut Vec<u8>) -> Result<(), FormatError>;
    fn pack_output(&self, output: &mut Vec<u8>);
}

pub fn select_format(name: FormatType) -> Box<dyn Format> {
    match name {
        FormatType::Raw => Box::new(self::raw::RawFormat {}),
        FormatType::Plain1 => Box::new(self::plain1::Plain1Format {}),
        FormatType::Hex => Box::new(self::hex::HexFormat {}),
        FormatType::Signal1 => Box::new(self::signal1::SignalFormat {}),
    }
}
