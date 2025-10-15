use crate::args::Args;
use crate::converters::ngram_to_string;
use crate::file_handling::{open_input, open_key, open_ngram, open_output, save_to_file};
use crate::file_parsers::{input_parser, key_parser, ngram_parser};
use crate::generators::histogram_generator;
use crate::operating_mode::OperatingMode;

pub fn encryption_decryption(args: Args, operating_mode: OperatingMode) {
    let input = args.input.unwrap();
    let output = args.output.unwrap();
    let key = args.key.unwrap();

    let input = open_input(input).expect("Failed to open input file");
    let output = open_output(output).expect("Failed to open output file");
    let key = open_key(key).expect("Failed to open key file");

    let input = input_parser(input);
    let key = key_parser(key, &operating_mode);

    let buf: String = input.chars().map(|x| key.get(&x).unwrap()).collect();

    save_to_file(&buf, output);
}

pub fn ngram_generator(args: Args) {
    let input = args.input.unwrap();
    let output = args.ngram_file.unwrap();
    let ngram_size = args.mode_group.gram.unwrap();

    let input = open_input(input).expect("Failed to open input file");
    let output = open_output(output).expect("Failed to open output file");

    let input = input_parser(input);

    let ngram = crate::generators::ngram_generator(&input, ngram_size);
    let histogram = histogram_generator(ngram);
    let buf = ngram_to_string(histogram);

    println!("{buf}");

    save_to_file(&buf, output);
}

pub fn ngram_reader(args: Args) {
    let input = args.ngram_file.unwrap();
    let ngram_size = args.mode_group.read_ngram.unwrap();

    let ngram = open_ngram(input).expect("Failed to open ngram file");

    let ngram = ngram_parser(ngram, ngram_size);

    println!("{}", ngram_to_string(ngram));
}

pub fn x2test(args: Args) {
    let input = args.input.unwrap();
    let ngram_ref = args.ngram_file.unwrap();
    let ngram_size = args.mode_group.read_ngram.unwrap();

    let input = open_input(input).expect("Failed to open input file");

    let input = input_parser(input);
    let ngram = crate::generators::ngram_generator(&input, ngram_size);
    let ngram = histogram_generator(ngram);

    let ngram_ref = open_ngram(ngram_ref).expect("Failed to open ngram file");
    let ngram_ref = ngram_parser(ngram_ref, ngram_size);

    let mut sum: f64 = 0.0;

    let n: u64 = ngram.iter().map(|(_, num)| num).sum();

    for i in 0..ngram.len() {
        let e = ngram_ref[i].1 * n as f64;
        sum += (ngram[i].1 as f64 - e).powi(2) / e
    }

    println!("{sum:.20}")
}
