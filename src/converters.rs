use std::fmt::Display;

/// Serialises an n-gram histogram into a newline separated `key: value` list.
pub fn ngram_to_string<T: Display>(input: Vec<(String, T)>) -> String {
    // Format each entry as "GRAM: VALUE" and concatenate the lines into a single string.
    input
        .iter()
        .map(|(k, v)| format!("{k} {v}"))
        .collect::<Vec<_>>()
        .join("\n")
}
