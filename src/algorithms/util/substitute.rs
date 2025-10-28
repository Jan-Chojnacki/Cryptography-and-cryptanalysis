use std::collections::HashMap;

/// Podstawia każdy znak wejściowy zgodnie z dostarczonym odwzorowaniem klucza.
pub fn substitute(input: &str, key: &HashMap<char, char>) -> String {
    input.chars().map(|x| key.get(&x).unwrap()).collect()
}
