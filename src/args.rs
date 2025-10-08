#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub input: String,
    #[arg(short, long)]
    pub output: String,
    #[arg(short, long)]
    pub key: String,
    #[clap(flatten)]
    pub mode_group: ModeGroup,
}

#[derive(clap::Args, Debug)]
#[group(required = true, multiple = false)]
pub struct ModeGroup {
    #[arg(short, long)]
    pub encrypt: bool,
    #[arg(short, long)]
    pub decrypt: bool,
}
