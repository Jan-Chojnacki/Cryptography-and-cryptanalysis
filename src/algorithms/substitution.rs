use std::collections::HashMap;
use std::path::PathBuf;
use crate::file_handling::{open_input, open_key, open_output, save_to_file};
use crate::file_parsers::{input_parser, key_parser};

pub fn handle_encrypt(input: PathBuf, output: PathBuf, key: PathBuf) {
    // Obtain handles to the plaintext, output and substitution key files.
    let input = open_input(input).expect("Failed to open input file");
    let output = open_output(output).expect("Failed to open output file");
    let key = open_key(key).expect("Failed to open key file");

    // Parse the raw files into their in-memory representations.
    let input = input_parser(input);
    let key = key_parser(key, false);

    // Substitute each character according to the key mapping.
    let buf: String = encrypt(&input, key);

    // Persist the transformed text to the requested destination.
    save_to_file(&buf, output);
}

pub fn encrypt(input: &str, key: HashMap<char, char>) -> String {
    input.chars().map(|x| key.get(&x).unwrap()).collect()
}

pub fn handle_decrypt(input: PathBuf, output: PathBuf, key: PathBuf) {
    // Obtain handles to the plaintext, output and substitution key files.
    let input = open_input(input).expect("Failed to open input file");
    let output = open_output(output).expect("Failed to open output file");
    let key = open_key(key).expect("Failed to open key file");

    // Parse the raw files into their in-memory representations.
    let input = input_parser(input);
    let key = key_parser(key, true);

    // Substitute each character according to the key mapping.
    let buf: String = decrypt(&input, key);

    // Persist the transformed text to the requested destination.
    save_to_file(&buf, output);
}

pub fn decrypt(input: &str, key: HashMap<char, char>) -> String {
    input.chars().map(|x| key.get(&x).unwrap()).collect()
}