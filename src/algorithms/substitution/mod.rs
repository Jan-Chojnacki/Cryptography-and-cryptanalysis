use crate::algorithms::util::substitute::substitute;
use crate::file_handling::{open_input, open_key, open_output, save_to_file};
use crate::file_parsers::{input_parser, key_parser};
use std::path::PathBuf;

pub fn handle_encrypt(input: PathBuf, output: PathBuf, key: PathBuf) {
    let input = open_input(input).expect("Failed to open input file");
    let output = open_output(output).expect("Failed to open output file");
    let key = open_key(key).expect("Failed to open key file");


    let input = input_parser(input);
    let key = key_parser(key, false);


    let buf: String = substitute(&input, &key);


    save_to_file(&buf, output);
}

pub fn handle_decrypt(input: PathBuf, output: PathBuf, key: PathBuf) {
    let input = open_input(input).expect("Failed to open input file");
    let output = open_output(output).expect("Failed to open output file");
    let key = open_key(key).expect("Failed to open key file");


    let input = input_parser(input);
    let key = key_parser(key, true);


    let buf: String = substitute(&input, &key);


    save_to_file(&buf, output);
}