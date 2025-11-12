use crate::algorithms::input_parser::input_parser;
use crate::algorithms::transposition::generate_transposition_key::generate_transposition_key;
use crate::algorithms::util::substitute::substitute;
use crate::file_handling::{open_input, open_output, save_to_file};
use std::path::PathBuf;

/// Wczytuje dane, odwraca przesunięcie klucza i zapisuje odszyfrowany tekst.
///
/// # Arguments
/// * `input` - Ścieżka do pliku z szyfrogramem generowanym szyfrem przestawieniowym.
/// * `output` - Ścieżka do pliku, w którym zapisany zostanie tekst jawny.
/// * `key` - Wartość przesunięcia cyklicznego alfabetu w zakresie 1-25.
pub fn handle_decrypt(input: PathBuf, output: PathBuf, key: u8) {
    let input = open_input(input).expect("Failed to open input file");
    let output = open_output(output).expect("Failed to open output file");

    let input = input_parser(input);

    let key = generate_transposition_key(-(key as i16));

    let buf: String = substitute(&input, &key);

    save_to_file(&buf, output);
}
