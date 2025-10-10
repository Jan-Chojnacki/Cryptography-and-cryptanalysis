mod args;
mod file_handling;
mod histogram_generator;
mod input_parser;
mod key_parser;
mod ngram_generator;
mod ngram_parser;
mod operating_mode;
mod validate_paths;

use crate::args::Args;
use crate::file_handling::{open_input, open_key, open_output};
use crate::histogram_generator::histogram_generator;
use crate::input_parser::input_parser;
use crate::key_parser::key_parser;
use crate::ngram_generator::ngram_generator;
use crate::ngram_parser::ngram_parser;
use crate::operating_mode::OperatingMode;
use clap::Parser;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let args = Args::parse();
    args.validate().expect("Validation failed");

    let operating_mode = args.operating_mode();

    match operating_mode {
        OperatingMode::ENCRYPTION => {
            let input = open_input(args.input.unwrap()).expect("Failed to open input file");
            let mut output = open_output(args.output.unwrap()).expect("Failed to open output file");
            let key = open_key(args.key.unwrap()).expect("Failed to open key file");

            let input = input_parser(input);
            let key = key_parser(key, &operating_mode);

            let buf = input;

            output
                .write_all(buf.as_bytes())
                .expect(format!("Could not write to output file at: {:?}.", output).as_str());
        }
        OperatingMode::DECRYPTION => {
            let input = OpenOptions::new()
                .read(true)
                .open(args.input.unwrap())
                .expect("Failed to open input file");

            let mut output = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(args.output.unwrap())
                .expect("Failed to open output file");

            let key = OpenOptions::new()
                .read(true)
                .open(args.key.unwrap())
                .expect("Failed to open key file");

            let input = input_parser(input);
            let key = key_parser(key, &operating_mode);

            let buf = input;

            output
                .write_all(buf.as_bytes())
                .expect(format!("Could not write to output file at: {:?}.", output).as_str());
        }
        OperatingMode::NgramGenerator => {
            let input = OpenOptions::new()
                .read(true)
                .open(args.input.unwrap())
                .expect("Failed to open input file");

            let mut output = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(args.output.unwrap())
                .expect("Failed to open output file");

            let input = input_parser(input);

            let ngram = ngram_generator(&input, args.mode_group.gram.unwrap());
            let histogram = histogram_generator(ngram);
            let buf = ngram_parser(histogram);

            println!("{buf}");

            output
                .write_all(buf.as_bytes())
                .expect(format!("Could not write to output file at: {:?}.", output).as_str());
        }
        OperatingMode::NgramReader => {}
    }
}
