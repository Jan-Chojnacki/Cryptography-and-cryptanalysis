//! Operacje analityczne i narzędziowe wykorzystywane przez aplikację z poziomu CLI.

use crate::converters::ngram_to_string;
use crate::file_handling::{open_input, open_ngram, open_output, save_to_file};
use crate::file_parsers::{input_parser, ngram_parser};
use crate::generators::histogram_generator;
use statrs::distribution::{ChiSquared, ContinuousCDF};
use std::path::PathBuf;

/// Generuje histogram n-gramów z tekstu wejściowego i opcjonalnie zapisuje go do pliku.
///
/// # Arguments
/// * `input` - Ścieżka do pliku z tekstem poddawanym analizie n-gramowej.
/// * `file` - Opcjonalna ścieżka do pliku wynikowego; `None` oznacza wyłącznie wypisanie w konsoli.
/// * `g` - Rozmiar n-gramów wykorzystywanych podczas generowania statystyk.
pub fn ngram_generator(input: PathBuf, file: Option<PathBuf>, g: u8) {
    let input = open_input(input).expect("Failed to open input file");

    let input = input_parser(input);

    let ngram = crate::generators::ngram_generator(&input, g);
    let histogram = histogram_generator(ngram);
    let buf = ngram_to_string(histogram);

    println!("{buf}");

    if let Some(file) = file {
        let output = open_output(file).expect("Failed to open output file");

        save_to_file(&buf, output);
    }
}

/// Wczytuje i wypisuje referencyjne statystyki n-gramów.
///
/// # Arguments
/// * `file` - Ścieżka do pliku z zapisanymi n-gramami w formacie tekstowym.
/// * `r` - Rozmiar n-gramów wykorzystywany podczas analizy statystycznej.
pub fn ngram_reader(file: PathBuf, r: u8) {
    let ngram = open_ngram(file).expect("Failed to open ngram file");

    let ngram = ngram_parser(ngram, r);

    println!("{}", ngram_to_string(ngram));
}

/// Oblicza statystykę chi-kwadrat dla porównania tekstu wejściowego z referencją n-gramów.
///
/// # Arguments
/// * `input` - Ścieżka do pliku z tekstem poddawanym analizie n-gramowej.
/// * `file` - Ścieżka do pliku zawierającego referencyjne częstotliwości n-gramów.
/// * `r` - Rozmiar n-gramów wykorzystywany podczas analizy statystycznej.
pub fn x2test(input: PathBuf, file: PathBuf, r: u8) {
    let input = open_input(input).expect("Failed to open input file");

    let input = input_parser(input);
    let ngram = crate::generators::ngram_generator(&input, r);
    let ngram = histogram_generator(ngram);

    let ngram_ref = open_ngram(file).expect("Failed to open ngram file");
    let ngram_ref = ngram_parser(ngram_ref, r);

    let mut x2: f64 = 0.0;

    let n: u64 = ngram.values().sum();

    for (k, v) in ngram {
        if let Some(rv) = ngram_ref.get(&k) {
            let e = rv * n as f64;
            x2 += (v as f64 - e).powi(2) / e;
        }
    }

    let df = 26.0f64.powi(r as i32) - 1.0;

    let chi = ChiSquared::new(df).expect("invalid degrees of freedom");

    let critical = chi.inverse_cdf(0.95f64);

    let reject_h0 = x2 >= critical;

    println!(
        "chi2_stat={:.12}, df={}, critical={:.12}, reject_H0={}",
        x2, df as u64, critical, reject_h0
    );
}
