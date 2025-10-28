use std::collections::HashMap;

/// Creates a rolling window of n-grams from the provided input text.
pub fn ngram_generator(input: &str, ngram_size: u8) -> Vec<String> {
    // Slide over the bytes to capture every n-length subsequence.
    input
        .as_bytes()
        .windows(ngram_size as usize)
        .map(|w| String::from_utf8_lossy(w).to_string())
        .collect()
}

/// Aggregates a collection of n-grams into a histogram sorted by frequency.
pub fn histogram_generator(ngram: Vec<String>) -> HashMap<String, u64> {
    ngram
        .iter()
        .fold(HashMap::<String, u64>::new(), |mut acc, gram| {
            *acc.entry(gram.clone()).or_insert(0) += 1;
            acc
        })
}