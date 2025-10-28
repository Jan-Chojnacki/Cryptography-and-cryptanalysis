mod algorithms;
mod args;
mod converters;
mod file_handling;
mod file_parsers;
mod generators;
mod operations;
mod attack;

use crate::algorithms::*;
use crate::args::{AlgorithmCommand, Args, AttackAlgorithmCommand, AttackCommand, Commands, NgramCommand};
use clap::Parser;
use crate::attack::*;

/// Entrypoint that parses CLI arguments, validates them and dispatches the
/// selected operating mode.
fn main() {
    let args = Args::parse();

    match args.commands {
        Commands::Encrypt { algorithm_command } => match algorithm_command {
            AlgorithmCommand::Substitution { input, output, key } => {
                substitution::handle_encrypt(input, output, key);
            }
            AlgorithmCommand::Transposition { input, output, key } => {
                transposition::handle_encrypt(input, output, key);
            }
            AlgorithmCommand::Affine {
                input,
                output,
                a,
                b,
            } => {
                affine::handle_encrypt(input, output, a, b);
            }
        },
        Commands::Decrypt { algorithm_command } => match algorithm_command {
            AlgorithmCommand::Substitution { input, output, key } => {
                substitution::handle_decrypt(input, output, key);
            }
            AlgorithmCommand::Transposition { input, output, key } => {
                transposition::handle_decrypt(input, output, key);
            }
            AlgorithmCommand::Affine {
                input,
                output,
                a,
                b,
            } => {
                affine::handle_decrypt(input, output, a, b);
            }
        },
        Commands::Ngram { ngram_command } => match ngram_command {
            NgramCommand::Generate { g, input, file } => {
                operations::ngram_generator(input, file, g);
            }
            NgramCommand::Read { r, file } => {
                operations::ngram_reader(file, r);
            }
        },
        Commands::Attack { attack_command } => match attack_command {
            AttackCommand::BruteForce { algorithm } => match algorithm {
                AttackAlgorithmCommand::Substitution { .. } => {

                }
                AttackAlgorithmCommand::Transposition { input, output, file, r } => {
                    bruteforce::transposition::handle_attack(input, output, file, r);
                }
                AttackAlgorithmCommand::Affine { .. } => {

                }
            },
        },
        Commands::Similarity { r, input, file } => {
            operations::x2test(input, file, r);
        }
    }
}
