use clap::{value_t, App, Arg, ArgMatches, SubCommand};
use std::fs::File;
use std::io::{Read, Write};
use std::io;

use sigli::{
    generate_key,
    encrypt,
    decrypt,
    SigliError,
    AlgoType,
    FormatType,
    ALL_FORMAT_NAMES,
    KEY_FORMAT_NAMES,
    DEFAULT_KEY_FORMAT,
    DEFAULT_PLAIN_FORMAT,
    DEFAULT_CIPHER_FORMAT,
    ALGORITHM_NAMES,
    DEFAULT_ALGO_NAME
};


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

#[derive(Debug)]
enum CliError {
    SigliError(SigliError),
    NoCommand,
    Io(io::Error),

}

impl std::convert::From<SigliError> for CliError {
    fn from(e: SigliError) -> Self {  CliError::SigliError(e) }
}

impl std::convert::From<io::Error> for CliError {
    fn from(e: io::Error) -> Self {  CliError::Io(e) }
}

fn read_file(file_name: &str) -> Result<Vec<u8>, CliError> {
    let mut file = File::open(file_name)?;

    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    Ok(data)
}

fn read_stdin() -> Result<Vec<u8>, CliError> {
    let mut buf = Vec::new();
    std::io::stdin().read_to_end(&mut buf)?;
    Ok(buf)
}

fn read_input(c: &ArgMatches) -> Result<Vec<u8>, CliError> {
    if let Some(file_name) = c.value_of(INPUT_ARG) {
        read_file(file_name)
    } else {
        read_stdin()
    }
}

fn read_key_data(c: &ArgMatches) -> Result<Vec<u8>, CliError> {
    let file_name = c.value_of(KEY_FILE_ARG).unwrap();
    read_file(file_name)
}

fn write_stdout(data: &[u8]) -> Result<(), CliError> {
    std::io::stdout().write_all(&data)?;
    Ok(())
}

fn write_file(file_name: &str, data: &[u8]) -> Result<(), CliError> {
    let mut file = File::create(file_name)?;
    file.write_all(data)?;
    Ok(())
}

fn write_output(c: &ArgMatches, data: &[u8]) -> Result<(), CliError> {
    if let Some(file_name) = c.value_of(OUTPUT_ARG) {
        write_file(file_name, &data)
    } else {
        write_stdout(&data)
    }
}


fn body() -> Result<(), CliError> {
    let m = App::new("Cipher CLI")
        .version(VERSION)
        .author("Richard Sunderland <richard@sunderlandfamily.info>")
        .about("Encrypt and decrypt short messages")
        .arg(Arg::with_name(ALGO_ARG)
            .long("--algo")
            .short("a")
            .value_name("ALGORITHM_NAME")
            .possible_values(ALGORITHM_NAMES)
            .default_value(DEFAULT_ALGO_NAME)
            .help(&format!("Name of algorithm."))
        )
        .arg(Arg::with_name(KEY_FORMAT_ARG)
            .long("--key-format")
            .short("K")
            .value_name("FORMAT_NAME")
            .possible_values(KEY_FORMAT_NAMES)
            .default_value(DEFAULT_KEY_FORMAT)
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
                .possible_values(ALL_FORMAT_NAMES)
                .default_value(DEFAULT_PLAIN_FORMAT)
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
                .possible_values(ALL_FORMAT_NAMES)
                .default_value(DEFAULT_CIPHER_FORMAT)
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
                .possible_values(ALL_FORMAT_NAMES)
                .default_value(DEFAULT_CIPHER_FORMAT)
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
                .possible_values(ALL_FORMAT_NAMES)
                .default_value(DEFAULT_PLAIN_FORMAT)
                .help(&format!("Plain text format."))
            )
        ).get_matches();


    let algo_type = value_t!(m.value_of(ALGO_ARG), AlgoType).unwrap();
    let key_format = value_t!(m.value_of(KEY_FORMAT_ARG), FormatType).unwrap();

    match m.subcommand() {
        (ENCRYPT_CMD, Some(c)) => {
            let input_format = value_t!(c.value_of(INPUT_FORMAT_ARG), FormatType).unwrap();
            let output_format = value_t!(c.value_of(OUTPUT_FORMAT_ARG), FormatType).unwrap();
            let raw_key = read_key_data(c)?;
            let raw_input = read_input(c)?;
            let raw_output = encrypt(
                algo_type,
                key_format,
                input_format,
                output_format,
                &raw_key,
                &raw_input,
            )?;
            write_output(c, &raw_output)
        }

        (DECRYPT_CMD, Some(c)) => {
            let input_format = value_t!(c.value_of(INPUT_FORMAT_ARG), FormatType).unwrap();
            let output_format = value_t!(c.value_of(OUTPUT_FORMAT_ARG), FormatType).unwrap();
            let raw_key = read_key_data(c)?;
            let raw_input = read_input(c)?;
            let raw_output = decrypt(
                algo_type,
                key_format,
                input_format,
                output_format,
                &raw_key,
                &raw_input,
            )?;
            write_output(c, &raw_output)
        }

        (GEN_KEY_CMD, Some(c)) => {
            let raw_key = generate_key(algo_type, key_format)?;
            write_output(c, &raw_key)
        }

        ("", None) => Err(CliError::NoCommand),
        _ => unreachable!(),
    }
}


fn main() {
    body().unwrap()
}
