mod args;
mod key_parser;
mod operating_mode;
mod input_parser;
mod ngram_generator;

use std::collections::HashMap;
use crate::args::Args;
use clap::Parser;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use crate::input_parser::input_parser;
use crate::key_parser::key_parser;
use crate::ngram_generator::ngram_generator;
use crate::operating_mode::OperatingMode;

fn main() {
    let args = Args::parse();

    let input = Path::new(&args.input);
    if input.extension().and_then(|ext| ext.to_str()) != Some("txt") {
        panic!("Only files with .txt extension are supported.");
    }

    let output = Path::new(&args.output);
    if output.extension().and_then(|ext| ext.to_str()) != Some("txt") {
        panic!("Only files with .txt extension are supported.");
    }

    let key = Path::new(&args.key);
    if key.extension().and_then(|ext| ext.to_str()) != Some("txt") {
        panic!("Only files with .txt extension are supported.");
    }

    let operating_mode = match (args.mode_group.decrypt, args.mode_group.encrypt, args.mode_group.gram) {
        (true, _, _) => OperatingMode::DECRYPTION,
        (_, true, _) => OperatingMode::ENCRYPTION,
        (_, _, n) => OperatingMode::NGRAM,
    };

    let input = OpenOptions::new()
        .read(true)
        .open(input)
        .expect("Failed to open input file");

    let mut output = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output)
        .expect("Failed to open output file");

    let key = OpenOptions::new()
        .read(true)
        .open(key)
        .expect("Failed to open key file");

    let input = input_parser(input);
    let key = key_parser(key, &operating_mode);

    let buf;

    match operating_mode {
        OperatingMode::ENCRYPTION => {
            buf = input;
        }
        OperatingMode::DECRYPTION => {
            buf = input;
        }
        OperatingMode::NGRAM => {
            buf = ngram_generator(&input, args.mode_group.gram.unwrap())
                .iter()
                .fold(HashMap::new(), |mut acc, gram| {
                    *acc.entry(gram).or_insert(0) += 1;
                    acc
                })
                .iter()
                .map(|(gram, count)| format!("{} {}", gram, count))
                .collect::<Vec<_>>()
                .join("\n");
        }
    }

    output
        .write_all(buf.as_bytes())
        .expect(format!("Could not write to output file at: {:?}.", output).as_str());
}
