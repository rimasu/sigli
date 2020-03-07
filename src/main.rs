mod core;
mod format;

use crate::core::{select_algorithm, CoreError, ALGORITHM_NAMES, AlgoType, DEFAULT_NAME};
use clap::{value_t, App, Arg, ArgMatches, SubCommand};
use std::fs::File;
use std::io::{Read, Write};
use crate::format::{FORMAT_NAMES, HEX_NAME, select_format, FormatError, FormatType, KEY_FORMAT_NAMES, PLAIN1_NAME, SIGNAL_NAME, Format};

#[derive(Debug)]
pub enum CliError {
    Core(CoreError),
    Format(FormatError),
    CouldNotCreateFile(String),
    CouldNotOpenFile(String),
    CouldNotReadFile(String),
    BadInput(String),
    BadOutput(String),
    NoCommand,
}

impl std::convert::From<CoreError> for CliError {
    fn from(e: CoreError) -> Self {
        CliError::Core(e)
    }
}

impl std::convert::From<FormatError> for CliError {
    fn from(e: FormatError) -> Self {
        CliError::Format(e)
    }
}

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

const GEN_KEY_CMD: &'static str = "genkey";
const ENCRYPT_CMD: &'static str = "encrypt";
const DECRYPT_CMD: &'static str = "decrypt";
const ALGO_ARG: &'static str = "algo";
const INPUT_ARG: &'static str = "input";
const OUTPUT_ARG: &'static str = "output";
const KEY_FILE_ARG: &'static str = "keyfile";
const KEY_FORMAT_ARG: &'static str = "keyformat";
const INPUT_FORMAT_ARG: &'static str = "inputformat";
const OUTPUT_FORMAT_ARG: &'static str = "outputformat";

fn read_file(file_name: &str) -> Result<Vec<u8>, CliError> {
    let mut file =
        File::open(file_name).map_err(|e| CliError::CouldNotOpenFile(format!("{}", e)))?;

    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .map_err(|e| CliError::CouldNotReadFile(format!("{}", e)))?;

    Ok(data)
}

fn read_stdin() -> Result<Vec<u8>, CliError> {
    let mut buf = Vec::new();
    std::io::stdin()
        .read_to_end(&mut buf)
        .map_err(|e| CliError::BadInput(format!("{}", e)))?;

    Ok(buf)
}

fn read_input(cmd_matches: &ArgMatches) -> Result<Vec<u8>, CliError> {
    let raw = if let Some(file_name) = cmd_matches.value_of(INPUT_ARG) {
        read_file(file_name)
    } else {
        read_stdin()
    }?;

    let format = read_format(cmd_matches, INPUT_FORMAT_ARG)?;
    let unpacked = format.unpack(&raw)?;
    Ok(unpacked)
}

fn read_key_data(cmd: &ArgMatches) -> Result<Vec<u8>, CliError> {
    let file_name = cmd.value_of(KEY_FILE_ARG).unwrap();
    read_file(file_name)
}

fn write_stdout(data: &[u8]) -> Result<(), CliError> {
    std::io::stdout()
        .write_all(&data)
        .map_err(|e| CliError::BadInput(format!("{}", e)))?;
    Ok(())
}

fn write_file(file_name: &str, data: &[u8]) -> Result<(), CliError> {
    let mut file =
        File::create(file_name).map_err(|e| CliError::CouldNotCreateFile(format!("{}", e)))?;

    file.write_all(data)
        .map_err(|e| CliError::BadInput(format!("{}", e)))?;
    Ok(())
}

fn write_output(cmd_matches: &ArgMatches, data: &[u8]) -> Result<(), CliError> {
    if let Some(file_name) = cmd_matches.value_of(OUTPUT_ARG) {
        write_file(file_name, &data)
    } else {
        write_stdout(&data)
    }
}

fn body() -> Result<(), CliError> {
    let matches = App::new("Cipher CLI")
        .version(VERSION)
        .author("Richard Sunderland <richard@sunderlandfamily.info>")
        .about("Encrypt and decrypt short messages")
        .arg(Arg::with_name(ALGO_ARG)
            .long("--algo")
            .short("a")
            .value_name("ALGORITHM_NAME")
            .possible_values(ALGORITHM_NAMES)
            .default_value(DEFAULT_NAME)
            .help(&format!("Name of algorithm."))
        )
        .arg(Arg::with_name(KEY_FORMAT_ARG)
            .long("--key-format")
            .short("K")
            .value_name("FORMAT_NAME")
            .possible_values(KEY_FORMAT_NAMES)
            .default_value(HEX_NAME)
            .help(&format!("Name of format."))
        )
        .subcommand(SubCommand::with_name(GEN_KEY_CMD)
            .about("generate new key")
            .arg(Arg::with_name(OUTPUT_ARG)
                .long("--output")
                .short("o")
                .value_name("KEY_FILE")
                .required(false)
                .help("Output file containing generated key. If absent output is written to stdout."))
        )
        .subcommand(SubCommand::with_name(ENCRYPT_CMD)
            .about("encrypt a message with an existing cipher")
            .arg(Arg::with_name(KEY_FILE_ARG)
                .value_name("KEY_FILE")
                .required(true)
                .help("File containing key data"))

            .arg(Arg::with_name(INPUT_ARG)
                .long("--input")
                .short("i")
                .value_name("PLAIN_TEXT_FILE")
                .required(false)
                .help("Input file containing plain text. If absent input is read from stdin."))
            .arg(Arg::with_name(INPUT_FORMAT_ARG)
                .long("--input-format")
                .short("I")
                .value_name("FORMAT_NAME")
                .possible_values(FORMAT_NAMES)
                .default_value(PLAIN1_NAME)
                .help(&format!("Plain text format."))
            )
            .arg(Arg::with_name(OUTPUT_ARG)
                .long("--output")
                .short("o")
                .value_name("CIPHER_TEXT_FILE")
                .required(false)
                .help("Output file containing cipher text. If absent output is written to stdout."))
            .arg(Arg::with_name(OUTPUT_FORMAT_ARG)
                .long("--output-format")
                .short("O")
                .value_name("FORMAT_NAME")
                .possible_values(FORMAT_NAMES)
                .default_value(SIGNAL_NAME)
                .help(&format!("Cipher text format."))
            )
        )
        .subcommand(SubCommand::with_name(DECRYPT_CMD)
            .about("decrypt a message with an existing cipher")
            .arg(Arg::with_name(KEY_FILE_ARG)
                .value_name("KEY_FILE")
                .required(true)
                .help("File containing key data"))
            .arg(Arg::with_name(INPUT_ARG)
                .long("--input")
                .short("i")
                .value_name("CIPHER_TEXT_FILE")
                .required(false)
                .help("Input file containing cipher text. If absent input is read from stdin."))
            .arg(Arg::with_name(INPUT_FORMAT_ARG)
                .long("--input-format")
                .short("I")
                .value_name("FORMAT_NAME")
                .possible_values(FORMAT_NAMES)
                .default_value(SIGNAL_NAME)
                .help(&format!("Cipher text format."))
            )
            .arg(Arg::with_name(OUTPUT_ARG)
                .long("--output")
                .short("o")
                .value_name("PLAIN_TEXT_FILE")
                .required(false)
                .help("Output file containing plain text. If absent output is written to stdout."))
            .arg(Arg::with_name(OUTPUT_FORMAT_ARG)
                .long("--output-format")
                .short("O")
                .value_name("FORMAT_NAME")
                .possible_values(FORMAT_NAMES)
                .default_value(PLAIN1_NAME)
                .help(&format!("Plain text format."))
            )
        ).get_matches();


    let algo_type = value_t!(matches.value_of(ALGO_ARG), AlgoType).unwrap();
    let algo = select_algorithm(algo_type)?;

    let key_format = read_format(&matches, KEY_FORMAT_ARG)?;

    match matches.subcommand() {
        (ENCRYPT_CMD, Some(cmd_matches)) => {
            let input = read_input(cmd_matches)?;
            let key_data = read_key_data(cmd_matches)?;
            let key_data = key_format.unpack(&key_data)?;
            let output = algo.encrypt_data(&key_data, &input)?;
            let output_format = read_format(cmd_matches, OUTPUT_FORMAT_ARG)?;
            let output = output_format.pack(&output)?;
            write_output(cmd_matches, &output)
        }

        (DECRYPT_CMD, Some(cmd_matches)) => {
            let input = read_input(cmd_matches)?;
            let key_data = read_key_data(cmd_matches)?;
            let key_data = key_format.unpack(&key_data)?;
            let output = algo.decrypt_data(&key_data, &input)?;
            let output_format = read_format(cmd_matches, OUTPUT_FORMAT_ARG)?;
            let output = output_format.pack(&output)?;
            write_output(cmd_matches, &output)
        }

        (GEN_KEY_CMD, Some(cmd_matches)) => {
            let output = algo.generate_key_data();
            let output = key_format.pack(&output)?;
            write_output(cmd_matches, &output)
        }

        ("", None) => Err(CliError::NoCommand),
        _ => unreachable!(),
    }
}

fn read_format(m: &ArgMatches, arg: &str) -> Result<Box<dyn Format>, CliError> {
    let format_type = value_t!(m.value_of(arg), FormatType).unwrap();
    let format = select_format(format_type)?;
    Ok(format)
}

fn main() {
    body().unwrap()
}
