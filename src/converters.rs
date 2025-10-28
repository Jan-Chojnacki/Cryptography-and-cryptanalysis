use std::collections::HashMap;
use std::fmt::Display;

/// Konwertuje mapę n-gramów na czytelną reprezentację tekstową z wierszami `klucz wartość`.
pub fn ngram_to_string<T: Display>(input: HashMap<String, T>) -> String {
    input
        .iter()
        .map(|(k, v)| format!("{k} {v}"))
        .collect::<Vec<_>>()
        .join("\n")
}
