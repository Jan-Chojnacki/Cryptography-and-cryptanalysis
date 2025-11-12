use crate::algorithms::input_parser::input_parser;
use crate::algorithms::substitution::key_parser::key_parser;
use crate::algorithms::util::substitute::substitute;
use crate::file_handling::{open_input, open_key, open_output, save_to_file};
use std::path::PathBuf;

/// Wczytuje tekst oraz klucz podstawieniowy i zapisuje zaszyfrowany rezultat.
///
/// # Arguments
/// * `input` - Ścieżka do pliku z tekstem jawnym przeznaczonym do zaszyfrowania.
/// * `output` - Ścieżka do pliku, w którym zapisany zostanie szyfrogram.
/// * `key` - Ścieżka do pliku z mapowaniem znaków reprezentującym klucz podstawieniowy.
pub fn handle_encrypt(input: PathBuf, output: PathBuf, key: PathBuf) {
    let input = open_input(input).expect("Failed to open input file");
    let output = open_output(output).expect("Failed to open output file");
    let key = open_key(key).expect("Failed to open key file");

    let input = input_parser(input);
    let key = key_parser(key, false);

    let buf: String = substitute(&input, &key);

    save_to_file(&buf, output);
}
