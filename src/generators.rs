//! Funkcje generujące n-gramy i ich histogramy wykorzystywane w analizie statystycznej.

use std::collections::HashMap;

/// Tworzy wektor wszystkich n-gramów o zadanym rozmiarze z tekstu wejściowego.
///
/// # Arguments
/// * `input` - Tekst źródłowy, z którego mają zostać wyodrębnione n-gramy.
/// * `ngram_size` - Liczba określająca długość pojedynczego n-gramu.
///
/// # Zwracana wartość
/// Zwraca wektor łańcuchów reprezentujących kolejne n-gramy w kolejności
/// występowania w tekście źródłowym.
pub fn ngram_generator(input: &str, ngram_size: u8) -> Vec<String> {
    input
        .as_bytes()
        .windows(ngram_size as usize)
        .map(|w| String::from_utf8_lossy(w).to_string())
        .collect()
}

/// Buduje histogram częstości występowania n-gramów.
///
/// # Arguments
/// * `ngram` - Wektor n-gramów, najczęściej otrzymany z `ngram_generator`.
///
/// # Zwracana wartość
/// Zwraca mapę zliczającą liczbę wystąpień każdego n-gramu w przekazanym wektorze.
pub fn histogram_generator(ngram: Vec<String>) -> HashMap<String, u64> {
    ngram
        .iter()
        .fold(HashMap::<String, u64>::new(), |mut acc, gram| {
            *acc.entry(gram.clone()).or_insert(0) += 1;
            acc
        })
}
