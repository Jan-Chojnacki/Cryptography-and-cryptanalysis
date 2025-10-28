pub mod generate_transposition_key;

use crate::algorithms::transposition::generate_transposition_key::generate_transposition_key;
use crate::algorithms::util::substitute::substitute;
use crate::file_handling::{open_input, open_output, save_to_file};
use crate::file_parsers::input_parser;
use std::path::PathBuf;

pub fn handle_encrypt(input: PathBuf, output: PathBuf, key: u8) {
    let input = open_input(input).expect("Failed to open input file");
    let output = open_output(output).expect("Failed to open output file");


    let input = input_parser(input);

    let key = generate_transposition_key(key as i16);


    let buf: String = substitute(&input, &key);


    save_to_file(&buf, output);
}

pub fn handle_decrypt(input: PathBuf, output: PathBuf, key: u8) {
    let input = open_input(input).expect("Failed to open input file");
    let output = open_output(output).expect("Failed to open output file");


    let input = input_parser(input);

    let key = generate_transposition_key(-(key as i16));


    let buf: String = substitute(&input, &key);


    save_to_file(&buf, output);
}