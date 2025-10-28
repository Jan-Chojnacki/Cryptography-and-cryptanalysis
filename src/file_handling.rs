use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

/// Otwiera plik wejściowy do odczytu, zwracając wynik lub błąd w formie tekstowej.
pub fn open_input(path: PathBuf) -> Result<File, String> {
    OpenOptions::new()
        .read(true)
        .open(path)
        .map_err(|e| format!("{:?}", e))
}

/// Tworzy lub nadpisuje plik wyjściowy, przygotowując go do zapisu wyników.
pub fn open_output(path: PathBuf) -> Result<File, String> {
    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .map_err(|e| format!("{:?}", e))
}

/// Otwiera plik z kluczem szyfrującym do odczytu.
pub fn open_key(path: PathBuf) -> Result<File, String> {
    OpenOptions::new()
        .read(true)
        .open(path)
        .map_err(|e| format!("{:?}", e))
}

/// Otwiera plik zawierający statystyki n-gramów do odczytu.
pub fn open_ngram(path: PathBuf) -> Result<File, String> {
    OpenOptions::new()
        .read(true)
        .open(path)
        .map_err(|e| format!("{:?}", e))
}

/// Zapisuje przekazaną zawartość tekstową do wskazanego pliku.
pub fn save_to_file(content: &str, mut output: File) {
    output
        .write_all(content.as_bytes())
        .expect("Could not write to output file");
}
