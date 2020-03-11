use assert_cmd::prelude::*;
use std::process::Command;
use predicates::str::{contains, is_empty};
use predicates::str::is_match;
use tempdir::TempDir;
use std::path::Path;

const HEX_128_BIT_PATTERN: &str = "^([A-F0-9]{4}-){7}[A-F0-9]{4}\n";
const HEX_256_BIT_PATTERN: &str = "^([A-F0-9]{4}-){15}[A-F0-9]{4}\n";
const SIGNAL1_PATTERN: &str = "^([A-Z]{5}\\s)*[A-Z]{1,5}";

#[test]
fn cli_no_args() {
    Command::cargo_bin("sigli").unwrap().assert().failure();
}

#[test]
fn cli_version() {
    Command::cargo_bin("sigli")
        .unwrap()
        .args(&["-V"])
        .assert()
        .stdout(contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn cli_genkey_to_stdout() {
    Command::cargo_bin("sigli")
        .unwrap()
        .args(&[
            "genkey"
        ])
        .assert()
        .stdout(is_match(HEX_256_BIT_PATTERN).unwrap());
}

#[test]
fn cli_genkey_to_stdout_with_algo_aes128gcm() {
    Command::cargo_bin("sigli")
        .unwrap()
        .args(&[
            "--algo",
            "aes128gcm",
            "genkey"
        ])
        .assert()
        .stdout(is_match(HEX_128_BIT_PATTERN).unwrap());
}

#[test]
fn cli_genkey_to_stdout_with_a_aes256gcm() {
    Command::cargo_bin("sigli")
        .unwrap()
        .args(&[
            "-a",
            "aes256gcm",
            "genkey"
        ])
        .assert()
        .stdout(is_match(HEX_256_BIT_PATTERN).unwrap());
}

#[test]
fn cli_genkey_to_file() {
    let dir = TempDir::new("sigli").unwrap();
    let output_file = dir.path().join("output_file");
    Command::cargo_bin("sigli")
        .unwrap()
        .args(&[
            "genkey",
            "--output",
            output_file.to_str().unwrap()
        ])
        .assert()
        .stdout(is_empty());

    assert_file_contents(output_file, HEX_256_BIT_PATTERN);
}

#[test]
fn cli_genkey_to_file_with_key_format_signal1() {
    let dir = TempDir::new("sigli").unwrap();
    let output_file = dir.path().join("output_file");
    Command::cargo_bin("sigli")
        .unwrap()
        .args(&[
            "--key-format",
            "signal1",
            "genkey",
            "--output",
            output_file.to_str().unwrap()
        ])
        .assert()
        .stdout(is_empty());

    assert_file_contents(&output_file, SIGNAL1_PATTERN);
    assert_file_length(&output_file, 66);
}

#[test]
fn cli_genkey_to_file_with_key_format_raw() {
    let dir = TempDir::new("sigli").unwrap();
    let output_file = dir.path().join("output_file");
    Command::cargo_bin("sigli")
        .unwrap()
        .args(&[
            "-K",
            "raw",
            "genkey",
            "-o",
            output_file.to_str().unwrap()
        ])
        .assert()
        .stdout(is_empty());

    assert_file_length(&output_file, 32);
}

fn assert_file_contents<P: AsRef<Path>>(path: P, pattern: &str) {
    assert!(path.as_ref().exists());
    let content = std::fs::read_to_string(path).unwrap();
    let re = regex::Regex::new(pattern).unwrap();
    debug_assert!(re.is_match(&content),
                  "File content did not match pattern.\nregex={}\ncontent:\n{}",
                  pattern, content
    );
}

fn assert_file_length<P: AsRef<Path>>(path: P, expected: usize) {
    assert!(path.as_ref().exists());
    let content = std::fs::read(path).unwrap();
    debug_assert!(content.len() == expected,
                  "File wrong length. expected={}, actual={}",
                  expected, content.len()
    );
}