mod args;
mod converters;
mod file_handling;
mod file_parsers;
mod generators;
mod operating_mode;
mod validate_paths;

use crate::args::Args;
use crate::converters::ngram_to_string;
use crate::file_handling::{open_input, open_key, open_ngram, open_output};
use crate::file_parsers::key_parser;
use crate::file_parsers::{input_parser, ngram_parser};
use crate::generators::{histogram_generator, ngram_generator};
use crate::operating_mode::OperatingMode;
use clap::Parser;
use std::io::Write;

fn main() {
    let args = Args::parse();
    args.validate().expect("Validation failed");

    let operating_mode = args.operating_mode();

    dbg!(&operating_mode);

    match operating_mode {
        OperatingMode::Encryption => {
            let input = args.input.unwrap();
            let output = args.output.unwrap();
            let key = args.key.unwrap();

            let input = open_input(input).expect("Failed to open input file");
            let mut output = open_output(output).expect("Failed to open output file");
            let key = open_key(key).expect("Failed to open key file");

            let input = input_parser(input);
            let key = key_parser(key, &operating_mode);

            let buf: String = input.chars().map(|x| key.get(&x).unwrap()).collect();

            output
                .write_all(buf.as_bytes())
                .expect(format!("Could not write to output file at: {:?}.", output).as_str());
        }
        OperatingMode::Decryption => {
            let input = args.input.unwrap();
            let output = args.output.unwrap();
            let key = args.key.unwrap();

            let input = open_input(input).expect("Failed to open input file");
            let mut output = open_output(output).expect("Failed to open output file");
            let key = open_key(key).expect("Failed to open key file");

            let input = input_parser(input);
            let key = key_parser(key, &operating_mode);

            let buf: String = input.chars().map(|x| key.get(&x).unwrap()).collect();

            output
                .write_all(buf.as_bytes())
                .expect(format!("Could not write to output file at: {:?}.", output).as_str());
        }
        OperatingMode::NgramGenerator => {
            let input = args.input.unwrap();
            let output = args.ngram_file.unwrap();
            let ngram_size = args.mode_group.gram.unwrap();

            let input = open_input(input).expect("Failed to open input file");
            let mut output = open_output(output).expect("Failed to open output file");

            let input = input_parser(input);

            let ngram = ngram_generator(&input, ngram_size);
            let histogram = histogram_generator(ngram);
            let buf = ngram_to_string(histogram);

            println!("{buf}");

            output
                .write_all(buf.as_bytes())
                .expect(format!("Could not write to output file at: {:?}.", output).as_str());
        }
        OperatingMode::NgramReader => {
            let input = args.ngram_file.unwrap();
            let ngram_size = args.mode_group.read_ngram.unwrap();

            let ngram = open_ngram(input).expect("Failed to open ngram file");

            let ngram = ngram_parser(ngram, ngram_size);

            println!("{}", ngram_to_string(ngram));
        }
        OperatingMode::X2Test => {
            let input = args.input.unwrap();
            let ngram_ref = args.ngram_file.unwrap();
            let ngram_size = args.mode_group.read_ngram.unwrap();

            let input = open_input(input).expect("Failed to open input file");

            let input = input_parser(input);
            let ngram = ngram_generator(&input, ngram_size);
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
    }
}
