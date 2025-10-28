use crate::algorithms::transposition;
use crate::attack::x2test::x2test;
use crate::file_handling::{open_input, open_ngram, open_output, save_to_file};
use crate::file_parsers::{input_parser, ngram_parser};
use crate::generators::{generate_transposition_key, histogram_generator};
use std::collections::HashMap;
use std::path::PathBuf;

pub fn handle_attack(input: PathBuf, output: PathBuf, ngram_ref: PathBuf, r: u8) {
    let input = open_input(input).expect("Failed to open input file");
    let input = input_parser(input);

    let ngram_ref = open_ngram(ngram_ref).expect("Failed to open ngram file");
    let ngram_ref = ngram_parser(ngram_ref, r);

    let df = 26.0f64.powi(r as i32) - 1.0;
    let p = 0.95f64;

    match attack(input, ngram_ref, df, p, r) {
        Ok(buf) => {
            let output = open_output(output).expect("Failed to open output file");
            save_to_file(&buf, output);
        }
        Err(_) => {
            println!("Failed to find key.");
        }
    }
}

fn attack(
    input: String,
    ngram_ref: HashMap<String, f64>,
    df: f64,
    p: f64,
    r: u8,
) -> Result<String, ()> {
    for i in 1..25u8 {
        let key = generate_transposition_key(-(i as i16));
        let decrypted = transposition::decrypt(&input, key);

        let ngram = crate::generators::ngram_generator(&decrypted, r);
        let ngram = histogram_generator(ngram);

        if x2test(&ngram, &ngram_ref, df, p) {
            println!("key={}", i);
            return Ok(decrypted);
        }
    }

    Err(())
}
