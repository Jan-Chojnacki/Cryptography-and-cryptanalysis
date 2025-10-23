use statrs::distribution::{ChiSquared, ContinuousCDF};
use crate::args::Args;
use crate::converters::ngram_to_string;
use crate::file_handling::{open_input, open_key, open_ngram, open_output, save_to_file};
use crate::file_parsers::{input_parser, key_parser, ngram_parser};
use crate::generators::histogram_generator;
use crate::operating_mode::OperatingMode;

/// Executes the classical substitution cipher in either encryption or decryption mode.
pub fn encryption_decryption(args: Args, operating_mode: OperatingMode) {
    // Extract the required file paths from the parsed arguments.
    let input = args.input.unwrap();
    let output = args.output.unwrap();
    let key = args.key.unwrap();

    // Obtain handles to the plaintext, output and substitution key files.
    let input = open_input(input).expect("Failed to open input file");
    let output = open_output(output).expect("Failed to open output file");
    let key = open_key(key).expect("Failed to open key file");

    // Parse the raw files into their in-memory representations.
    let input = input_parser(input);
    let key = key_parser(key, &operating_mode);

    // Substitute each character according to the key mapping.
    let buf: String = input.chars().map(|x| key.get(&x).unwrap()).collect();

    // Persist the transformed text to the requested destination.
    save_to_file(&buf, output);
}

/// Generates an n-gram histogram from an input file and stores it in both stdout and a file.
pub fn ngram_generator(args: Args) {
    // Gather the input path, output destination and requested n-gram size.
    let input = args.input.unwrap();
    let output = args.ngram_file.unwrap();
    let ngram_size = args.mode_group.gram.unwrap();

    // Read the plaintext input and prepare the output file.
    let input = open_input(input).expect("Failed to open input file");
    let output = open_output(output).expect("Failed to open output file");

    // Normalise the plaintext prior to n-gram extraction.
    let input = input_parser(input);

    // Build the n-gram list, convert it into a histogram and serialise the result.
    let ngram = crate::generators::ngram_generator(&input, ngram_size);
    let histogram = histogram_generator(ngram);
    let buf = ngram_to_string(histogram);

    println!("{buf}");

    // Write the histogram to disk for later use.
    save_to_file(&buf, output);
}

/// Prints a previously generated n-gram histogram to stdout.
pub fn ngram_reader(args: Args) {
    // Retrieve the histogram path and its associated n-gram size.
    let input = args.ngram_file.unwrap();
    let ngram_size = args.mode_group.read_ngram.unwrap();

    // Load and parse the histogram file to recover its probability distribution.
    let ngram = open_ngram(input).expect("Failed to open ngram file");

    let ngram = ngram_parser(ngram, ngram_size);

    println!("{}", ngram_to_string(ngram));
}

/// Performs the chi-squared goodness-of-fit test between two n-gram distributions.
pub fn x2test(args: Args) {
    // Obtain the ciphertext path, reference histogram and the n-gram size used for comparison.
    let input = args.input.unwrap();
    let ngram_ref = args.ngram_file.unwrap();
    let ngram_size = args.mode_group.read_ngram.unwrap();

    // Parse the ciphertext and compute its histogram for the requested n-gram length.
    let input = open_input(input).expect("Failed to open input file");

    let input = input_parser(input);
    let ngram = crate::generators::ngram_generator(&input, ngram_size);
    let ngram = histogram_generator(ngram);

    // Load the reference histogram used as the expected distribution.
    let ngram_ref = open_ngram(ngram_ref).expect("Failed to open ngram file");
    let ngram_ref = ngram_parser(ngram_ref, ngram_size);

    let mut sum: f64 = 0.0;

    // Total number of observed n-grams in the analysed text.
    let n: u64 = ngram.iter().map(|(_, num)| num).sum();

    // Apply the chi-squared formula across each n-gram observation.
    for i in 0..ngram.len() {
        let e = ngram_ref[i].1 * n as f64;
        sum += (ngram[i].1 as f64 - e).powi(2) / e
    }

    // Output the resulting chi-squared statistic for downstream analysis.
    println!("{sum:.20}")
}

pub fn attack(args: Args) {
    let df = 1.0;
    let cfd = ChiSquared::new(df).expect("TODO: panic message");
    let icfd = cfd.inverse_cdf(0.95);
}