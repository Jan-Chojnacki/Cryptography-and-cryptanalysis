use std::os::raw::c_long;
use std::path::PathBuf;

/// Program lab 1
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to input file.
    #[arg(short, long, value_name="FILE")]
    pub input: PathBuf,
    /// Path to output file.
    #[arg(short, long, value_name="FILE")]
    pub output: PathBuf,
    /// Path to key file.
    #[arg(short, long, value_name="FILE")]
    pub key: PathBuf,
    /// Program operation mode.
    #[clap(flatten)]
    pub mode_group: ModeGroup,
}

#[derive(clap::Args, Debug)]
#[group(required = true, multiple = false)]
pub struct ModeGroup {
    /// Encryption mode.
    #[arg(short, long)]
    pub encrypt: bool,
    /// Decryption mode.
    #[arg(short, long)]
    pub decrypt: bool,
    /// Monogram generation mode.
    #[arg(short, long, value_name = "NUMBER", value_parser = clap::value_parser!(u8).range(1..=4))]
    pub gram: Option<u8>,
}
