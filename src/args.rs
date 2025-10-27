use clap::Subcommand;
use std::path::PathBuf;

/// Command line arguments accepted by the application.
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub commands: Commands,
}
#[derive(Subcommand, Debug)]
#[command(infer_subcommands = true)]
pub enum Commands {
    Encrypt {
        #[command(subcommand)]
        algorithm: Algorithm,
    },
    Decrypt {
        #[command(subcommand)]
        algorithm: Algorithm,
    },
    Ngram {
        #[command(subcommand)]
        ngram_commands: NgramCommands,
    },
    Attack {

    },
    Similarity {
        #[arg(short, long, value_name = "NUMBER", value_parser = clap::value_parser!(u8).range(1..=4))]
        r: u8,
        #[arg(value_name = "FILE")]
        file: PathBuf,
        #[arg(short, long, value_name = "FILE")]
        input: PathBuf,
    }
}

#[derive(Subcommand, Debug)]
pub enum NgramCommands {
    Generate {
        #[arg(short, long, value_name = "NUMBER", value_parser = clap::value_parser!(u8).range(1..=4))]
        g: u8,
        #[arg(short, long, value_name = "FILE")]
        input: PathBuf,
        #[arg(value_name = "FILE")]
        file: Option<PathBuf>,
    },
    Read {
        #[arg(short, long, value_name = "NUMBER", value_parser = clap::value_parser!(u8).range(1..=4))]
        r: u8,
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
}

#[derive(Subcommand, Debug)]
pub enum Algorithm {
    Substitution {
        #[clap(flatten)]
        args: EncryptionDecryptionArgsKeyText,
    },
    Transposition {
        #[clap(flatten)]
        args: EncryptionDecryptionArgsKeyNumeric,
    },
    Affine {

    },
}

#[derive(clap::Args, Debug)]
pub struct EncryptionDecryptionArgsKeyText {
    #[arg(short, long, value_name = "FILE")]
    pub input: PathBuf,
    #[arg(short, long, value_name = "FILE")]
    pub output: PathBuf,
    #[arg(short, long, value_name = "FILE")]
    pub key: PathBuf,
}

#[derive(clap::Args, Debug)]
pub struct EncryptionDecryptionArgsKeyNumeric {
    #[arg(short, long, value_name = "FILE")]
    pub input: PathBuf,
    #[arg(short, long, value_name = "FILE")]
    pub output: PathBuf,
    #[arg(short, long)]
    pub key: u8,
}