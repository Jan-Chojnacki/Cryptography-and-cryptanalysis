use crate::attack::x2test::x2test;
use crate::file_handling::{open_input, open_ngram, open_output, save_to_file};
use crate::file_parsers::{input_parser, ngram_parser};
use crate::generators::{histogram_generator, ngram_generator};
use rayon::prelude::*;
use statrs::distribution::{ChiSquared, ContinuousCDF};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;
use crate::algorithms::transposition::generate_transposition_key::generate_transposition_key;
use crate::algorithms::util::substitute::substitute;

pub fn handle_attack(input: PathBuf, output: PathBuf, ngram_ref: PathBuf, r: u8) {
    let input = open_input(input).expect("Failed to open input file");
    let input = input_parser(input);

    let ngram_ref = open_ngram(ngram_ref).expect("Failed to open ngram file");
    let ngram_ref = ngram_parser(ngram_ref, r);

    let df = 26.0f64.powi(r as i32) - 1.0;
    let p = 0.95f64;

    let buf = attack(input, ngram_ref, df, p, r);
    let output = open_output(output).expect("Failed to open output file");
    save_to_file(&buf, output);
}

fn attack(input: String, ngram_ref: HashMap<String, f64>, df: f64, p: f64, r: u8) -> String {
    let ngram = ngram_generator(&input, r);
    let n = ngram.len() as f64;
    let ngram_ref = ngram_ref.iter().map(|(k, v)| (k.clone(), v * n)).collect();

    let chi = ChiSquared::new(df).expect("invalid degrees of freedom");
    let critical = chi.inverse_cdf(p);

    let results: Mutex<Vec<(u8, f64)>> = Mutex::new(Vec::new());

    if let Some((i, decrypted)) = (1u8..=25)
        .into_par_iter()
        .filter_map(|i| {
            let key = generate_transposition_key(-(i as i16));
            let decrypted = substitute(&input, key);

            let ngram = ngram_generator(&decrypted, r);
            let ngram = histogram_generator(ngram);

            match x2test(&ngram, &ngram_ref, critical) {
                Ok(_) => Some((i, decrypted)),
                Err(x2) => {
                    results.lock().unwrap().push((i, x2));
                    None
                }
            }
        })
        .find_any(|_| true)
    {
        println!("key={}", i);
        return decrypted;
    }

    println!("Failed to find key.");
    let mut results = results.into_inner().unwrap();
    results.sort_by(|a, b| a.1.total_cmp(&b.1));

    let (best_key, best_x2) = results.first().unwrap();
    println!("best_key={}, best_x2={}", best_key, best_x2);
    let key = generate_transposition_key(-(*best_key as i16));
    substitute(&input, key)
}
