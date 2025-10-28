/// Tworzy wektor wszystkich n-gramów o zadanym rozmiarze z tekstu wejściowego.
pub fn ngram_generator(input: &str, ngram_size: u8) -> Vec<String> {
    input
        .as_bytes()
        .windows(ngram_size as usize)
        .map(|w| String::from_utf8_lossy(w).to_string())
        .collect()
}