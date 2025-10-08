mod args;
mod key_parser;
mod operating_mode;

use crate::args::Args;
use clap::Parser;
use std::fs::{File, OpenOptions};
use std::path::Path;
use crate::key_parser::key_parser;
use crate::operating_mode::OperatingMode;

fn main() {
    let args = Args::parse();

    let input = Path::new(&args.input);
    if input.extension().and_then(|ext| ext.to_str()) != Some("txt") {
        panic!("Only files with .txt extension are supported.");
    }
    if !input.exists() {
        panic!("Input path does not exist: {:?}", input);
    }

    let output = Path::new(&args.output);
    if output.extension().and_then(|ext| ext.to_str()) != Some("txt") {
        panic!("Only files with .txt extension are supported.");
    }
    if !output.exists() {
        println!("Output path does not exist: {:?}", output);
        OpenOptions::new().write(true).create_new(true).open(output).expect(&format!("Could not create file at: {:?}", output));
        println!("Creating output file at: {:?}", output);
    }

    let key = Path::new(&args.key);
    if key.extension().and_then(|ext| ext.to_str()) != Some("txt") {
        panic!("Only files with .txt extension are supported.");
    }
    if !key.exists() {
        panic!("Key path does not exist: {:?}", key);
    }

    let operating_mode = match (args.mode_group.decrypt, args.mode_group.encrypt) {
        (true, false) => OperatingMode::DECRYPTION,
        (false, true) => OperatingMode::ENCRYPTION,
        (_, _) => panic!("Only one operating mode can be selected at a time.")
    };

    let input = File::open(input).unwrap();
    let output = File::open(output).unwrap();
    let key = File::open(key).unwrap();

    let key = key_parser(key, operating_mode);

    println!("{:?}", args);
    println!("{:?}", input);
    println!("{:?}", output);
    println!("{:?}", key);
}
