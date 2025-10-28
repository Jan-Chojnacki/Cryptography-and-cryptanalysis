use std::fs::File;
use std::io::{BufRead, BufReader};

/// Czyta plik tekstowy, filtruje znaki do alfabetu łacińskiego i zwraca je w postaci wielkich liter.
///
/// # Arguments
/// * `input` - Uchwyt do pliku z tekstem źródłowym przeznaczonym do dalszego przetwarzania.
///
/// # Zwracana wartość
/// Zwraca łańcuch składający się wyłącznie z wielkich liter alfabetu ASCII, gotowy do
/// wykorzystania przez algorytmy szyfrujące oraz analizy statystyczne.
pub fn input_parser(input: File) -> String {
    let reader = BufReader::new(input);
    let mut buf: Vec<String> = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let filtered_string: String =
                line.chars().filter(|c| c.is_ascii_alphabetic()).collect();
            buf.push(filtered_string.to_uppercase())
        }
    }

    buf.join("")
}
