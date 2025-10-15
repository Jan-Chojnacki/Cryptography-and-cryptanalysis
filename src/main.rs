mod args;
mod converters;
mod file_handling;
mod file_parsers;
mod generators;
mod operating_mode;
mod validate_paths;
mod operations;

use crate::args::Args;
use crate::operating_mode::OperatingMode;
use clap::Parser;

fn main() {
    let args = Args::parse();
    args.validate().expect("Validation failed");

    let operating_mode = args.operating_mode();

    match operating_mode {
        OperatingMode::Encryption => {
            operations::encryption_decryption(args, operating_mode);
        }
        OperatingMode::Decryption => {
            operations::encryption_decryption(args, operating_mode);
        }
        OperatingMode::NgramGenerator => {
            operations::ngram_generator(args);
        }
        OperatingMode::NgramReader => {
            operations::ngram_reader(args);
        }
        OperatingMode::X2Test => {
            operations::x2test(args);
        }
    }
}
