pub mod generate_affine_encrypt_key;
pub mod generate_affine_decrypt_key;

use std::path::PathBuf;
use crate::algorithms::affine::generate_affine_decrypt_key::generate_affine_decrypt_key;
use crate::algorithms::affine::generate_affine_encrypt_key::generate_affine_encrypt_key;
use crate::algorithms::util::substitute::substitute;
use crate::file_handling::{open_input, open_output, save_to_file};
use crate::file_parsers::{input_parser};

pub fn handle_encrypt(input: PathBuf, output: PathBuf, a: u32, b: u32) {
    // Obtain handles to the plaintext, output and substitution key files.
    let input = open_input(input).expect("Failed to open input file");
    let output = open_output(output).expect("Failed to open output file");

    // Parse the raw files into their in-memory representations.
    let input = input_parser(input);
    let key = generate_affine_encrypt_key(a, b);

    // Substitute each character according to the key mapping.
    let buf: String = substitute(&input, &key);

    // Persist the transformed text to the requested destination.
    save_to_file(&buf, output);
}

pub fn handle_decrypt(input: PathBuf, output: PathBuf, a: u32, b: u32) {
    // Obtain handles to the plaintext, output and substitution key files.
    let input = open_input(input).expect("Failed to open input file");
    let output = open_output(output).expect("Failed to open output file");

    // Parse the raw files into their in-memory representations.
    let input = input_parser(input);
    let key = generate_affine_decrypt_key(a, b);

    // Substitute each character according to the key mapping.
    let buf: String = substitute(&input, &key);

    // Persist the transformed text to the requested destination.
    save_to_file(&buf, output);
}