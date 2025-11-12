use crate::algorithms::input_parser::input_parser;
use crate::algorithms::util::substitute::substitute;
use crate::analysis::fast_substitute::fast_substitute;
use crate::analysis::p::p;
use crate::analysis::pl::pl;
use crate::analysis::roll_key::roll_key;
use crate::file_handling::{open_input, open_ngram, open_output, save_to_file};
use crate::ngram::histogram_generator::histogram_generator;
use crate::ngram::histogram_parser::histogram_parser;
use crate::ngram::ngram_generator::ngram_generator;
use std::collections::HashMap;
use std::path::PathBuf;

pub fn handle_analysis(input: PathBuf, output: PathBuf, ngram_ref: PathBuf, t: usize) {
    let input = open_input(input).expect("Failed to open input file");
    let input = input_parser(input);

    let ngram_ref = open_ngram(ngram_ref).expect("Failed to open ngram file");
    let ngram_ref = histogram_parser(ngram_ref, 2);

    let buf = analysis(input, ngram_ref, t);
    let output = open_output(output).expect("Failed to open output file");
    save_to_file(&buf, output);
}

fn analysis(input: String, ngram_ref: HashMap<String, u64>, t: usize) -> String {
    let mut key: [char; 26] = [
        'A','B','C','D','E','F','G','H','I','J','K','L','M',
        'N','O','P','Q','R','S','T','U','V','W','X','Y','Z'
    ];
    let mut ngram_k = histogram_generator(ngram_generator(&fast_substitute(&input, &key), 2));

    // let i = AtomicUsize::new(0);
    // while i.load(Ordering::Acquire) <= t {
    //     if let Some((new_key, ri)) = rayon::iter::repeat(())
    //         .map(|_| roll_key(&key))
    //         .filter_map(|rk| step(&rk, &input, &ngram_k, &ngram_ref, &i))
    //         .find_any(|_| true)
    //     {
    //         key = if ri <= t { new_key } else { key };
    //         ngram_k = histogram_generator(ngram_generator(&substitute(&input, &key), 2));
    //         i.store(ri, Ordering::Release);
    //     }
    // }

    for _ in 0..t {
        let rk = roll_key(&key);
        if let Some(new_key) = step(&rk, &input, &ngram_k, &ngram_ref) {
            key = new_key;
            ngram_k = histogram_generator(ngram_generator(&fast_substitute(&input, &key), 2));
        }
    }

    fast_substitute(&input, &key)
}

// fn step(
//     rk: &HashMap<char, char>,
//     input: &str,
//     ngram_k: &HashMap<String, u64>,
//     ngram_ref: &HashMap<String, u64>,
//     i: &AtomicUsize,
// ) -> Option<(HashMap<char, char>, usize)> {
//     let ri = i.fetch_add(1, Ordering::Relaxed);
//
//     let pl_k = pl(&ngram_k, &ngram_ref, 0.01);
//
//     let ngram_rk = histogram_generator(ngram_generator(&substitute(&input, &rk), 2));
//     let pl_rk = pl(&ngram_rk, &ngram_ref, 0.01);
//
//     let p = p(pl_k, pl_rk);
//     let u: f64 = rand::random();
//
//     match u <= p {
//         true => Some((rk.clone(), ri)),
//         false => None,
//     }
// }

fn step(
    rk: &[char; 26],
    input: &str,
    ngram_k: &HashMap<String, u64>,
    ngram_ref: &HashMap<String, u64>,
) -> Option<[char; 26]> {
    let pl_k = pl(&ngram_k, &ngram_ref, 0.01);

    let ngram_rk = histogram_generator(ngram_generator(&fast_substitute(&input, &rk), 2));
    let pl_rk = pl(&ngram_rk, &ngram_ref, 0.01);

    let p = p(pl_k, pl_rk);
    let u: f64 = rand::random();

    match u <= p {
        true => Some(rk.clone()),
        false => None,
    }
}
