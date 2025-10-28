use std::collections::HashMap;

/// Buduje histogram częstości występowania n-gramów.
///
/// # Arguments
/// * `ngram` - Wektor n-gramów wygenerowany z badanego tekstu.
///
/// # Zwracana wartość
/// Zwraca mapę zliczeń, w której kluczem jest n-gram, a wartością liczba jego wystąpień.
pub fn histogram_generator(ngram: Vec<String>) -> HashMap<String, u64> {
    ngram
        .iter()
        .fold(HashMap::<String, u64>::new(), |mut acc, gram| {
            *acc.entry(gram.clone()).or_insert(0) += 1;
            acc
        })
}
