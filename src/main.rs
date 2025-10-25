mod args;
mod attacks;
mod converters;
mod file_handling;
mod file_parsers;
mod generators;
mod operations;

use crate::args::{Args, Commands, NgramCommands};
use clap::Parser;

/// Entrypoint that parses CLI arguments, validates them and dispatches the
/// selected operating mode.
fn main() {
    let args = Args::parse();

    match args.commands {
        Commands::Encrypt { input, output, key } => {
            operations::encryption_decryption(input, output, key, false);
        }
        Commands::Decrypt { input, output, key } => {
            operations::encryption_decryption(input, output, key, true);
        }
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
