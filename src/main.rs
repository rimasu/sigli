#[macro_use]
extern crate serde;

mod core;

use clap::{App, Arg, ArgMatches, SubCommand};
use crate::core::{select_algorithm, Algorithm, CoreError};
use std::fs::File;
use std::io::{Write, Read};

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
const KEY_FILE_ARG: &'static str = "keyfile";

fn load_key_data(cmd: &ArgMatches) -> Result<Vec<u8>, CliError> {
    let file_name = cmd.value_of(KEY_FILE_ARG).unwrap();
    let mut file = File::open(file_name)
        .map_err(|e| CliError::CouldNotOpenFile(format!("{}", e)))?;

    let mut key_data = Vec::new();
    file.read_to_end(&mut key_data)
        .map_err(|e| CliError::CouldNotReadFile(format!("{}", e)))?;

    Ok(key_data)
}

fn collect_stdin() -> Result<Vec<u8>, CliError> {
    let mut buf = Vec::new();
    std::io::stdin().read_to_end(&mut buf)
        .map_err(|e| CliError::BadInput(format!("{}", e)))?;

    Ok(buf)
}

fn write_stdout(data: &[u8]) -> Result<(), CliError> {
    std::io::stdout().write_all(&data)
        .map_err(|e| CliError::BadInput(format!("{}", e)))?;
    Ok(())
}

fn body() -> Result<(), CliError> {
    let matches =  App::new("Cipher CLI")
        .version(VERSION)
        .author("Richard Sunderland <richard@sunderlandfamily.info>")
        .about("Encrypt and decrypt short messages")
        .arg(Arg::with_name(ALGO_ARG)
            .long("--algo")
            .short("a")
            .takes_value(true)
            .help(&format!("Name of algorithm. Defaults to {}. Options are {}",
                           core::DEFAULT_NAME,
                           core::ALGORITHM_NAMES
                               .join(", ")
            ))
        )
        .subcommand(SubCommand::with_name(GEN_KEY_CMD)
            .about("generate new key")
            .arg(Arg::with_name("keyfile")
                .value_name("KEY_FILE")
                .takes_value(true)
                .help("File to which the key will be written. \
                 If absent the key will be written to the stdout."))
        )
        .subcommand(SubCommand::with_name(ENCRYPT_CMD)
            .about("encrypt a message with an existing cipher")
            .arg(Arg::with_name(KEY_FILE_ARG)
                .value_name("KEY_FILE")
                .required(true)
                .help("File containing key data"))
        )
        .subcommand(SubCommand::with_name(DECRYPT_CMD)
            .about("decrypt a message with an existing cipher")
            .arg(Arg::with_name(KEY_FILE_ARG)
                .value_name("KEY_FILE")
                .required(true)
                .help("File containing key data"))
        ).get_matches();

    let algo_name = matches.value_of(ALGO_ARG).unwrap_or(core::DEFAULT_NAME);
    let algo = select_algorithm(algo_name)?;

    match matches.subcommand() {
        (ENCRYPT_CMD, Some(cmd_matches)) => {
            let input = collect_stdin()?;
            let key_data = load_key_data(cmd_matches)?;
            let output = algo.encrypt_data(&key_data, &input)?;
            write_stdout(&output)
        }

        (DECRYPT_CMD, Some(cmd_matches)) => {
            let input = collect_stdin()?;
            let key_data = load_key_data(cmd_matches)?;
            let output = algo.decrypt_data(&key_data, &input)?;
            write_stdout(&output)
        }

        (GEN_KEY_CMD, Some(cmd_matches)) => {
            let output = algo.generate_key_text();
            write_stdout(&output)
        }

        ("", None) => Err(CliError::NoCommand), // If no subcommand was usd it'll match the tuple ("", None)
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }
}

fn main() {
    body().unwrap()
}
