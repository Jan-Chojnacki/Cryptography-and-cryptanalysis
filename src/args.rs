use crate::operating_mode::OperatingMode;
use std::path::PathBuf;

/// Program lab 1
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to input file.
    #[arg(short, long, value_name = "FILE")]
    pub input: Option<PathBuf>,
    /// Path to output file.
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,
    /// Path to key file.
    #[arg(short, long, value_name = "FILE")]
    pub key: Option<PathBuf>,
    /// Program operation mode.
    #[clap(flatten)]
    pub mode_group: ModeGroup,
    /// Path to ngram file.
    #[arg(value_name = "FILE", requires = "read_ngram")]
    pub read_ngram_file: Option<PathBuf>,
    /// Generating x^2 test.
    #[arg(short)]
    pub s: bool,
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
    /// Ngram generation mode.
    #[arg(short, long, value_name = "NUMBER", value_parser = clap::value_parser!(u8).range(1..=4))]
    pub gram: Option<u8>,
    /// Ngram reading mode.
    #[arg(short, long, value_name = "NUMBER", value_parser = clap::value_parser!(u8).range(1..=4))]
    pub read_ngram: Option<u8>,
}

impl Args {
    pub fn validate(&self) -> Result<(), String> {
        self.validate_paths()?;
        self.validate_args()?;

        Ok(())
    }

    pub fn operating_mode(&self) -> OperatingMode {
        match (
            self.mode_group.decrypt,
            self.mode_group.encrypt,
            self.s,
            self.mode_group.gram,
            self.mode_group.read_ngram,
        ) {
            (true, _, _, _, _) => OperatingMode::DECRYPTION,
            (_, true, _, _, _) => OperatingMode::ENCRYPTION,
            (_, _, true, _, _) => OperatingMode::X2TEST,
            (_, _, _, Some(_), _) => OperatingMode::NgramGenerator,
            (_, _, _, _, Some(_)) => OperatingMode::NgramReader,
            _ => unreachable!("Exactly one mode must be set by clap ArgGroup"),
        }
    }

    fn validate_paths(&self) -> Result<(), String> {
        if let Some(input) = &self.input {
            if input.extension().and_then(|ext| ext.to_str()) != Some("txt") {
                return Err("Only files with .txt extension are supported.".into());
            }
        }

        if let Some(output) = &self.output {
            if output.extension().and_then(|ext| ext.to_str()) != Some("txt") {
                return Err("Only files with .txt extension are supported.".into());
            }
        }

        if let Some(key) = &self.key {
            if key.extension().and_then(|ext| ext.to_str()) != Some("txt") {
                return Err("Only files with .txt extension are supported.".into());
            }
        }

        Ok(())
    }

    fn validate_args(&self) -> Result<(), String> {
        if self.mode_group.encrypt || self.mode_group.decrypt {
            if self.key.is_none() {
                return Err("Error: --key is required when using --encrypt or --decrypt.".into());
            }
        }

        if self.mode_group.gram.is_some() {
            if self.input.is_none() || self.output.is_none() {
                return Err("Error: --input and --output are required when using --gram.".into());
            }
            if self.key.is_some() {
                return Err("Error: --key is not supported when using --gram.".into());
            }
        }

        Ok(())
    }
}
