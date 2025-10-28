//! Funkcje generujące n-gramy i ich histogramy wykorzystywane w analizie statystycznej.

use std::collections::HashMap;

/// Tworzy wektor wszystkich n-gramów o zadanym rozmiarze z tekstu wejściowego.
pub fn ngram_generator(input: &str, ngram_size: u8) -> Vec<String> {
    input
        .as_bytes()
        .windows(ngram_size as usize)
        .map(|w| String::from_utf8_lossy(w).to_string())
        .collect()
}

/// Buduje histogram częstości występowania n-gramów.
pub fn histogram_generator(ngram: Vec<String>) -> HashMap<String, u64> {
    ngram
        .iter()
        .fold(HashMap::<String, u64>::new(), |mut acc, gram| {
            *acc.entry(gram.clone()).or_insert(0) += 1;
            acc
        })
}
