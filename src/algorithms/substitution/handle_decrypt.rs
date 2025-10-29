use std::path::PathBuf;
use crate::algorithms::input_parser::input_parser;
use crate::algorithms::substitution::key_parser::key_parser;
use crate::algorithms::util::substitute::substitute;
use crate::file_handling::{open_input, open_key, open_output, save_to_file};

/// Wczytuje tekst oraz klucz i zapisuje odszyfrowany tekst jawny.
///
/// # Arguments
/// * `input` - Ścieżka do pliku z szyfrogramem przeznaczonym do deszyfrowania.
/// * `output` - Ścieżka do pliku, w którym zapisany zostanie tekst jawny.
/// * `key` - Ścieżka do pliku z odwzorowaniem znaków, interpretowanym jako klucz.
pub fn handle_decrypt(input: PathBuf, output: PathBuf, key: PathBuf) {
    let input = open_input(input).expect("Failed to open input file");
    let output = open_output(output).expect("Failed to open output file");
    let key = open_key(key).expect("Failed to open key file");

    let input = input_parser(input);
    let key = key_parser(key, true);

    let buf: String = substitute(&input, &key);

    save_to_file(&buf, output);
}