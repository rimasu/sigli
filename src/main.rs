mod core;

use crate::core::{select_algorithm, CoreError};
use clap::{App, Arg, ArgMatches, SubCommand};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug)]
pub enum CliError {
    Core(CoreError),
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

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

const GEN_KEY_CMD: &'static str = "genkey";
const ENCRYPT_CMD: &'static str = "encrypt";
const DECRYPT_CMD: &'static str = "decrypt";
const ALGO_ARG: &'static str = "algo";
const INPUT_ARG: &'static str = "input";
const OUTPUT_ARG: &'static str = "output";
const KEY_FILE_ARG: &'static str = "keyfile";

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
    if let Some(file_name) = cmd_matches.value_of(INPUT_ARG) {
        read_file(file_name)
    } else {
        read_stdin()
    }
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
        write_file(file_name, data)
    } else {
        write_stdout(data)
    }
}

fn body() -> Result<(), CliError> {
    let matches = App::new("Cipher CLI")
        .version(VERSION)
        .author("Richard Sunderland <richard@sunderlandfamily.info>")
        .about("Encrypt and decrypt short messages")

        .subcommand(SubCommand::with_name(GEN_KEY_CMD)
            .about("generate new key")
            .arg(Arg::with_name(ALGO_ARG)
                .long("--algo")
                .short("a")
                .value_name("ALGORITHM_NAME")
                .takes_value(true)
                .help(&format!("Name of algorithm. Defaults to {}. Options are {}",
                               core::DEFAULT_NAME,
                               core::ALGORITHM_NAMES
                                   .join(", ")
                ))
            )
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
            .arg(Arg::with_name(ALGO_ARG)
                .long("--algo")
                .short("a")
                .value_name("ALGORITHM_NAME")
                .takes_value(true)
                .help(&format!("Name of algorithm. Defaults to {}. Options are {}",
                               core::DEFAULT_NAME,
                               core::ALGORITHM_NAMES
                                   .join(", ")
                ))
            )
            .arg(Arg::with_name(INPUT_ARG)
                .long("--input")
                .short("i")
                .value_name("PLAIN_TEXT_FILE")
                .required(false)
                .help("Input file containing plain text. If absent input is read from stdin."))
            .arg(Arg::with_name(OUTPUT_ARG)
                .long("--output")
                .short("o")
                .value_name("CIPHER_TEXT_FILE")
                .required(false)
                .help("Output file containing cipher text. If absent output is written to stdout."))
        )
        .subcommand(SubCommand::with_name(DECRYPT_CMD)
            .about("decrypt a message with an existing cipher")
            .arg(Arg::with_name(KEY_FILE_ARG)
                .value_name("KEY_FILE")
                .required(true)
                .help("File containing key data"))
            .arg(Arg::with_name(ALGO_ARG)
                .long("--algo")
                .short("a")
                .value_name("ALGORITHM_NAME")
                .takes_value(true)
                .help(&format!("Name of algorithm. Defaults to {}. Options are {}",
                               core::DEFAULT_NAME,
                               core::ALGORITHM_NAMES
                                   .join(", ")
                ))
            )
            .arg(Arg::with_name(INPUT_ARG)
                .long("--input")
                .short("i")
                .value_name("CIPHER_TEXT_FILE")
                .required(false)
                .help("Input file containing cipher text. If absent input is read from stdin."))
            .arg(Arg::with_name(OUTPUT_ARG)
                .long("--output")
                .short("o")
                .value_name("PLAIN_TEXT_FILE")
                .required(false)
                .help("Output file containing plain text. If absent output is written to stdout."))
        ).get_matches();

    let algo_name = matches.value_of(ALGO_ARG).unwrap_or(core::DEFAULT_NAME);

    let algo = select_algorithm(algo_name)?;

    match matches.subcommand() {
        (ENCRYPT_CMD, Some(cmd_matches)) => {
            let input = read_input(cmd_matches)?;
            let key_data = read_key_data(cmd_matches)?;
            let output = algo.encrypt_data(&key_data, &input)?;
            write_output(cmd_matches, &output)
        }

        (DECRYPT_CMD, Some(cmd_matches)) => {
            let input = read_input(cmd_matches)?;
            let key_data = read_key_data(cmd_matches)?;
            let output = algo.decrypt_data(&key_data, &input)?;
            write_output(cmd_matches, &output)
        }

        (GEN_KEY_CMD, Some(cmd_matches)) => {
            let output = algo.generate_key_text();
            write_output(cmd_matches, &output)
        }

        ("", None) => Err(CliError::NoCommand),
        _ => unreachable!(),
    }
}

fn main() {
    body().unwrap()
}
