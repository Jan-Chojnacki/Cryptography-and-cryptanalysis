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
pub fn histogram_generator(ngram: Vec<String>) -> Vec<(String, u64)> {
    // Count occurrences of each n-gram using a hash map accumulator.
    let mut res = ngram
        .iter()
        .fold(HashMap::new(), |mut acc, gram| {
            *acc.entry(gram.clone()).or_insert(0) += 1;
            acc
        })
        // Move the aggregated counts into a vector to preserve ordering requirements.
        .iter()
        .fold(Vec::new(), |mut acc, (k, v)| {
            acc.push((k.clone(), *v));
            acc
        });

    // Sort the histogram by frequency in descending order.
    res.sort_by_key(|&(_, v)| v);
    res.reverse();

    res
}
