use crate::converters::ngram_to_string;
use crate::file_handling::{open_input, open_ngram, open_output, save_to_file};
use crate::file_parsers::{input_parser, ngram_parser};
use crate::generators::histogram_generator;
use statrs::distribution::{ChiSquared, ContinuousCDF};
use std::path::PathBuf;


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


pub fn ngram_reader(file: PathBuf, r: u8) {
    let ngram = open_ngram(file).expect("Failed to open ngram file");

    let ngram = ngram_parser(ngram, r);

    println!("{}", ngram_to_string(ngram));
}


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



