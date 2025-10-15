/// Application modes that dictate how the input files are processed.
#[derive(Debug)]
pub enum OperatingMode {
    Encryption,
    Decryption,
    NgramGenerator,
    NgramReader,
    X2Test,
}
