mod args;
mod key_parser;
mod operating_mode;
mod input_parser;

use crate::args::Args;
use clap::Parser;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use crate::input_parser::input_parser;
use crate::key_parser::key_parser;
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

    let operating_mode = match (args.mode_group.decrypt, args.mode_group.encrypt) {
        (true, false) => OperatingMode::DECRYPTION,
        (false, true) => OperatingMode::ENCRYPTION,
        (_, _) => panic!("Only one operating mode can be selected at a time.")
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
    let key = key_parser(key, operating_mode);

    output
        .write_all(input.as_bytes())
        .expect(format!("Could not write to output file at: {:?}.", output).as_str());
}
