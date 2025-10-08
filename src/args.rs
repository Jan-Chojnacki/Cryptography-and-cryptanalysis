/// Program lab 1
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to input file.
    #[arg(short, long)]
    pub input: String,
    /// Path to output file.
    #[arg(short, long)]
    pub output: String,
    /// Path to key file.
    #[arg(short, long)]
    pub key: String,
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
}
