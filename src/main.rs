mod algorithms;
mod args;
mod attacks;
mod converters;
mod file_handling;
mod file_parsers;
mod generators;
mod operations;

use crate::algorithms::*;
use crate::args::{Algorithm, Args, Commands, NgramCommands};
use clap::Parser;

/// Entrypoint that parses CLI arguments, validates them and dispatches the
/// selected operating mode.
fn main() {
    let args = Args::parse();

    match args.commands {
        Commands::Encrypt { algorithm } => match algorithm {
            Algorithm::Substitution { input, output, key } => {
                substitution::handle_encrypt(input, output, key);
            }
            Algorithm::Transposition { input, output, key } => {
                transposition::handle_encrypt(input, output, key);
            }
            Algorithm::Affine {input, output, a, b} => {
                affine::handle_encrypt(input, output, a, b);
            }
        },
        Commands::Decrypt { algorithm } => match algorithm {
            Algorithm::Substitution { input, output, key } => {
                substitution::handle_decrypt(input, output, key);
            }
            Algorithm::Transposition { input, output, key } => {
                transposition::handle_decrypt(input, output, key);
            }
            Algorithm::Affine {input, output, a, b} => {
                affine::handle_decrypt(input, output, a, b);
            }
        },
        Commands::Ngram { ngram_commands } => match ngram_commands {
            NgramCommands::Generate { g, input, file } => {
                operations::ngram_generator(input, file, g);
            }
            NgramCommands::Read { r, file } => {
                operations::ngram_reader(file, r);
            }
        },
        Commands::Attack { .. } => {}
        Commands::Similarity { r, input, file } => {
            operations::x2test(input, file, r);
        }
    }
}
