use std::path::PathBuf;
use crate::algorithms::affine::generate_affine_encrypt_key::generate_affine_encrypt_key;
use crate::algorithms::input_parser::input_parser;
use crate::algorithms::util::substitute::substitute;
use crate::file_handling::{open_input, open_output, save_to_file};

/// Wczytuje dane wejściowe, generuje klucz szyfrujący `Ax + B mod 26` i zapisuje wynik.
///
/// # Arguments
/// * `input` - Ścieżka do pliku z tekstem jawnym przeznaczonym do zaszyfrowania.
/// * `output` - Ścieżka do pliku, w którym zapisany zostanie szyfrogram.
/// * `a` - Współczynnik multiplikatywny klucza afinicznego wymagający istnienia odwrotności modulo 26.
/// * `b` - Współczynnik addytywny klucza afinicznego, redukowany do zakresu alfabetu (mod 26).
pub fn handle_encrypt(input: PathBuf, output: PathBuf, a: u32, b: u32) {
    let input = open_input(input).expect("Failed to open input file");
    let output = open_output(output).expect("Failed to open output file");

    let input = input_parser(input);
    let key = generate_affine_encrypt_key(a, b);

    let buf: String = substitute(&input, &key);

    save_to_file(&buf, output);
}