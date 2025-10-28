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
        algorithm_command: AlgorithmCommand,
    },
    Decrypt {
        #[command(subcommand)]
        algorithm_command: AlgorithmCommand,
    },
    Ngram {
        #[command(subcommand)]
        ngram_command: NgramCommand,
    },
    Attack {
        #[command(subcommand)]
        attack_command: AttackCommand,
    },
    Similarity {
        #[arg(short, long, value_name = "NUMBER", value_parser = clap::value_parser!(u8).range(1..=4))]
        r: u8,
        #[arg(value_name = "FILE")]
        file: PathBuf,
        #[arg(short, long, value_name = "FILE")]
        input: PathBuf,
    },
}

#[derive(Subcommand, Debug)]
pub enum NgramCommand {
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
pub enum AlgorithmCommand {
    Substitution {
        #[arg(short, long, value_name = "FILE")]
        input: PathBuf,
        #[arg(short, long, value_name = "FILE")]
        output: PathBuf,
        #[arg(short, long, value_name = "FILE")]
        key: PathBuf,
    },
    Transposition {
        #[arg(short, long, value_name = "FILE")]
        input: PathBuf,
        #[arg(short, long, value_name = "FILE")]
        output: PathBuf,
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=25))]
        key: u8,
    },
    Affine {
        #[arg(short, long, value_name = "FILE")]
        input: PathBuf,
        #[arg(short, long, value_name = "FILE")]
        output: PathBuf,
        #[arg(short, long)]
        a: u32,
        #[arg(short, long)]
        b: u32,
    },
}

#[derive(Subcommand, Debug)]
#[command(infer_subcommands = true)]
pub enum AttackCommand {
    BruteForce {
        #[command(subcommand)]
        algorithm: AttackAlgorithmCommand,
    },
}

#[derive(Subcommand, Debug)]
pub enum AttackAlgorithmCommand {
    Substitution {
        #[arg(short, long, value_name = "FILE")]
        input: PathBuf,
        #[arg(short, long, value_name = "FILE")]
        output: PathBuf,
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=4))]
        r: u8,
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
    Transposition {
        #[arg(short, long, value_name = "FILE")]
        input: PathBuf,
        #[arg(short, long, value_name = "FILE")]
        output: PathBuf,
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=4))]
        r: u8,
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
    Affine {
        #[arg(short, long, value_name = "FILE")]
        input: PathBuf,
        #[arg(short, long, value_name = "FILE")]
        output: PathBuf,
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=4))]
        r: u8,
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
}