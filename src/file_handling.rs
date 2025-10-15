use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

/// Opens a text file for reading and returns a read-only handle.
pub fn open_input(path: PathBuf) -> Result<File, String> {
    OpenOptions::new()
        .read(true)
        .open(path)
        .map_err(|e| format!("{:?}", e))
}

/// Opens (or creates) a text file for writing, truncating existing content.
pub fn open_output(path: PathBuf) -> Result<File, String> {
    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .map_err(|e| format!("{:?}", e))
}

/// Opens a substitution key file for reading.
pub fn open_key(path: PathBuf) -> Result<File, String> {
    OpenOptions::new()
        .read(true)
        .open(path)
        .map_err(|e| format!("{:?}", e))
}

/// Opens a precomputed n-gram histogram for reading.
pub fn open_ngram(path: PathBuf) -> Result<File, String> {
    OpenOptions::new()
        .read(true)
        .open(path)
        .map_err(|e| format!("{:?}", e))
}

/// Writes the provided text into the supplied file handle.
pub fn save_to_file(content: &str, mut output: File) {
    // Persist the serialised data to disk, surfacing any I/O error to the caller.
    output
        .write_all(content.as_bytes())
        .expect("Could not write to output file");
}
