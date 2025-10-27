use std::path::PathBuf;
use statrs::distribution::{ChiSquared, ContinuousCDF};
use crate::converters::ngram_to_string;
use crate::file_handling::{open_input, open_key, open_ngram, open_output, save_to_file};
use crate::file_parsers::{input_parser, key_parser, ngram_parser};
use crate::generators::histogram_generator;

/// Executes the classical substitution cipher in either encryption or decryption mode.
pub fn encryption_decryption(input: PathBuf, output: PathBuf, key: PathBuf, decryption: bool) {
    // Obtain handles to the plaintext, output and substitution key files.
    let input = open_input(input).expect("Failed to open input file");
    let output = open_output(output).expect("Failed to open output file");
    let key = open_key(key).expect("Failed to open key file");

    // Parse the raw files into their in-memory representations.
    let input = input_parser(input);
    let key = key_parser(key, decryption);

    // Substitute each character according to the key mapping.
    let buf: String = input.chars().map(|x| key.get(&x).unwrap()).collect();

    // Persist the transformed text to the requested destination.
    save_to_file(&buf, output);
}

/// Generates an n-gram histogram from an input file and stores it in both stdout and a file.
pub fn ngram_generator(input: PathBuf, file: Option<PathBuf>, g: u8) {
    // Read the plaintext input.
    let input = open_input(input).expect("Failed to open input file");

    // Normalise the plaintext prior to n-gram extraction.
    let input = input_parser(input);

    // Build the n-gram list, convert it into a histogram and serialise the result.
    let ngram = crate::generators::ngram_generator(&input, g);
    let histogram = histogram_generator(ngram);
    let buf = ngram_to_string(histogram);

    println!("{buf}");

    if let Some(file) = file {
        let output = open_output(file).expect("Failed to open output file");
        // Write the histogram to disk for later use.
        save_to_file(&buf, output);
    }
}

/// Prints a previously generated n-gram histogram to stdout.
pub fn ngram_reader(file: PathBuf, r: u8) {
    // Load and parse the histogram file to recover its probability distribution.
    let ngram = open_ngram(file).expect("Failed to open ngram file");

    let ngram = ngram_parser(ngram, r);

    println!("{}", ngram_to_string(ngram));
}

/// Performs the chi-squared goodness-of-fit test between two n-gram distributions.
pub fn x2test(input: PathBuf, file: PathBuf, r: u8) {
    // Parse the ciphertext and compute its histogram for the requested n-gram length.
    let input = open_input(input).expect("Failed to open input file");

    let input = input_parser(input);
    let ngram = crate::generators::ngram_generator(&input, r);
    let ngram = histogram_generator(ngram);

    // Load the reference histogram used as the expected distribution.
    let ngram_ref = open_ngram(file).expect("Failed to open ngram file");
    let ngram_ref = ngram_parser(ngram_ref, r);

    let mut x2: f64 = 0.0;
    let mut k_used = 0usize;

    // Total number of observed n-grams in the analysed text.
    let n: u64 = ngram.values().sum();

    // Apply the chi-squared formula across each n-gram observation.
    for (k, v) in ngram {
        if let Some(rv) = ngram_ref.get(&k) {
            let e = rv * n as f64;
            x2 += (v as f64 - e).powi(2) / e;
            k_used += 1;
        }
    }

    let df = if k_used > 0 { (k_used as i64 - 1).max(1) as f64 } else { 1.0 };

    let alpha: f64 = 0.05;

    let chi = ChiSquared::new(df).expect("invalid degrees of freedom");

    let q = (1.0 - alpha).clamp(1e-12, 1.0 - 1e-12);
    let critical = chi.inverse_cdf(q);

    let reject_h0 = x2 >= critical;

    println!(
        "chi2_stat={:.12}, df={}, critical={:.12}, reject_H0={}",
        x2, df as u64, critical, reject_h0
    );
}

// pub fn attack() {
//
// }