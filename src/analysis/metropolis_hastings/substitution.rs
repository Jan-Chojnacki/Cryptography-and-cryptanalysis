use crate::algorithms::input_parser::input_parser;
use crate::analysis::fast_substitute::fast_substitute;
use crate::analysis::p::p;
use crate::analysis::pl::pl;
use crate::analysis::roll_key::roll_key;
use crate::file_handling::{open_input, open_ngram, open_output, save_to_file};
use std::path::PathBuf;
use crate::analysis::fast_ngram_generator::fast_ngram_generator;
use crate::analysis::fast_ngram_parser::fast_ngram_parser;

pub fn handle_analysis(input: PathBuf, output: PathBuf, ngram_ref: PathBuf, t: usize) {
    let input = open_input(input).expect("Failed to open input file");
    let input = input_parser(input);

    let ngram_ref = open_ngram(ngram_ref).expect("Failed to open ngram file");
    let ngram_ref = fast_ngram_parser(ngram_ref);

    let buf = analysis(input, ngram_ref, t);
    let output = open_output(output).expect("Failed to open output file");
    save_to_file(&buf, output);
}

fn analysis(input: String, ngram_ref: [[f32; 26]; 26], t: usize) -> String {
    let mut key: [u8; 26] = [
        b'A',b'B',b'C',b'D',b'E',b'F',b'G',b'H',b'I',b'J',b'K',b'L',b'M',
        b'N',b'O',b'P',b'Q',b'R',b'S',b'T',b'U',b'V',b'W',b'X',b'Y',b'Z'
    ];
    let mut ngram_k = fast_ngram_generator(&fast_substitute(&input, &key));

    let mut rng = rand::thread_rng();

    for _ in 0..t {
        let rk = roll_key(&key, &mut rng);
        if let Some(new_key) = step(&rk, &input, &ngram_k, &ngram_ref) {
            key = new_key;
            ngram_k = fast_ngram_generator(&fast_substitute(&input, &key));
        }
    }

    fast_substitute(&input, &key)
}

fn step(
    rk: &[u8; 26],
    input: &str,
    ngram_k: &[[f32; 26]; 26],
    ngram_ref: &[[f32; 26]; 26],
) -> Option<[u8; 26]> {
    let pl_k = pl(&ngram_k, &ngram_ref, 0.01);

    let ngram_rk = fast_ngram_generator(&fast_substitute(&input, &rk));
    let pl_rk = pl(&ngram_rk, &ngram_ref, 0.01);

    let p = p(pl_k, pl_rk);
    let u: f32 = rand::random();

    match u <= p {
        true => Some(rk.clone()),
        false => None,
    }
}
