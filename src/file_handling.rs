use std::fs::{File, OpenOptions};
use std::path::PathBuf;

pub fn open_input(path: PathBuf) -> Result<File, String> {
    OpenOptions::new()
        .read(true)
        .open(path)
        .map_err(|e| format!("{:?}", e))
}

pub fn open_output(path: PathBuf) -> Result<File, String> {
    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .map_err(|e| format!("{:?}", e))
}

pub fn open_key(path: PathBuf) -> Result<File, String> {
    OpenOptions::new()
        .read(true)
        .open(path)
        .map_err(|e| format!("{:?}", e))
}

pub fn open_ngram(path: PathBuf) -> Result<File, String> {
    OpenOptions::new()
        .read(true)
        .open(path)
        .map_err(|e| format!("{:?}", e))
}