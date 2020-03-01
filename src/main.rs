#[macro_use]
extern crate serde;

mod core;

use clap::{App, Arg, ArgMatches, SubCommand};
use crate::core::{select_algorithm, Algorithm, CoreError};
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
pub enum CliError {
    Core(CoreError),
    CouldNotCreateFile(String),
    NoCommand,
}

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn genkey(algo: &Box<dyn Algorithm>, keyfile: Option<&str>) -> Result<(), CliError> {
    let key = algo.generate_key_text();
    if let Some(keyfile) = keyfile {
        let mut file = File::create(keyfile).map_err(|e|
            CliError::CouldNotCreateFile(format!("{}", e))
        )?;
        file.write_all(key.as_bytes());
    } else {
        println!("{}", key);
    }
    Ok(())
}

fn main() {
    let matches = App::new("Cipher CLI")
        .version(VERSION)
        .author("Richard Sunderland <richard@sunderlandfamily.info>")
        .about("Encrypt and decrypt short messages")
        .arg(Arg::with_name("algo")
            .long("--algo")
            .short("a")
            .takes_value(true)
            .help(&format!("Name of algorithm. Defaults to {}. Options are {}",
                           core::DEFAULT_NAME,
                           core::ALGORITHM_NAMES
                               .join(", ")
            ))
        )
        .subcommand(SubCommand::with_name("genkey")
            .about("generate new key")
            .arg(Arg::with_name("keyfile")
                .value_name("KEY_FILE")
                .takes_value(true)
                .help("File to which the key will be written. \
                 If absent the key will be written to the stdout."))
        )
        .subcommand(SubCommand::with_name("encrypt")
            .about("encrypt a message with an existing cipher")
            .arg(Arg::with_name("cipher")
                .value_name("CIPHER")
                .required(true)
                .help("Name of cipher to apply."))
            .arg(Arg::with_name("input")
                .value_name("PLAIN_TEXT_FILE")
                .required(true)
                .help("File containing plain text to encrypt.")))
        .subcommand(SubCommand::with_name("decrypt")
            .about("decrypt a message with an existing cipher")
            .arg(Arg::with_name("cipher")
                .long("cipher-name")
                .short("c")
                .value_name("CIPHER")
                .help("Name of cipher to apply."))
            .arg(Arg::with_name("input")
                .value_name("CIPHER_TEXT_FILE")
                .required(true)
                .help("File containing cipher text to decrypt. \
                If no file name given, input is taken from standard in.")))


        .get_matches();


    let algo_name = matches.value_of("algo").unwrap_or(core::DEFAULT_NAME);
    let algo = select_algorithm(algo_name).unwrap();

    let result = match matches.subcommand() {
        // ("encrypt", Some(cmd_matches)) => encrypt(&matches, &cmd_matches),
        //
        // ("decrypt", Some(cmd_matches)) => decrypt(&matches, &cmd_matches),

        ("genkey", Some(cmd_matches)) => {
            let keyfile = cmd_matches.value_of("keyfile");
            genkey(&algo, keyfile)
        }

        ("", None) => Err(CliError::NoCommand), // If no subcommand was usd it'll match the tuple ("", None)
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    };

    result.unwrap();
}
