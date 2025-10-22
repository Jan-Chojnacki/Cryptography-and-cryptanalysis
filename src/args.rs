use crate::operating_mode::OperatingMode;
use crate::operating_mode::OperatingMode::{
    Decryption, Encryption, NgramGenerator, NgramReader, X2Test,
};
use clap::ArgGroup;
use std::path::PathBuf;

/// Command line arguments accepted by the application.
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(group = ArgGroup::new("mode").args(["encrypt", "decrypt", "gram", "read_ngram"]))]
#[command(group = ArgGroup::new("ngram-file").args(["gram", "read_ngram"]))]
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
    #[arg(value_name = "FILE", requires = "ngram-file")]
    pub ngram_file: Option<PathBuf>,
}

/// Flags that identify the operating mode in which the application should run.
#[derive(clap::Args, Debug)]
pub struct ModeGroup {
    /// Encryption mode.
    #[arg(short, long, requires_all = ["input", "output", "key"])]
    pub encrypt: bool,
    /// Decryption mode.
    #[arg(short, long, requires_all = ["input", "output", "key"])]
    pub decrypt: bool,
    /// Ngram generation mode.
    #[arg(short, long, value_name = "NUMBER", value_parser = clap::value_parser!(u8).range(1..=4), requires_all = ["input", "ngram-file"])]
    pub gram: Option<u8>,
    /// Ngram reading mode.
    #[arg(short, long, value_name = "NUMBER", value_parser = clap::value_parser!(u8).range(1..=4), requires_all = ["ngram-file"])]
    pub read_ngram: Option<u8>,
    /// Generating x^2 test.
    #[arg(short, requires_all = ["read_ngram", "input"])]
    pub s: bool,
}

impl Args {
    /// Performs basic validation of the supplied paths and flags.
    pub fn validate(&self) -> Result<(), String> {
        // Ensure that each path argument uses a supported file extension.
        self.validate_paths()?;

        Ok(())
    }

    /// Determines which operating mode should be executed based on the provided flags.
    pub fn operating_mode(&self) -> OperatingMode {
        // Translate the clap-generated booleans into simple aliases for readability.
        let e = self.mode_group.encrypt;
        let d = self.mode_group.decrypt;
        let n = self.mode_group.gram.is_some();
        let r = self.mode_group.read_ngram.is_some();
        let s = self.mode_group.s;

        // Map the mutually exclusive combinations of flags to their semantic meaning.
        match (e, d, n, r, s) {
            (true, false, false, false, false) => Encryption,
            (false, true, false, false, false) => Decryption,
            (false, false, true, false, false) => NgramGenerator,
            (false, false, false, true, false) => NgramReader,
            (false, false, false, true, true) => X2Test,
            _ => panic!("Impossible combination of flags"),
        }
    }

    /// Validates that every provided path points to a `.txt` file.
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
}
