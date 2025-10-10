use std::collections::HashMap;

pub fn ngram_parser(input: HashMap<String, u64>) -> String {
    input
        .iter()
        .map(|(gram, count)| format!("{} {}", gram, count))
        .collect::<Vec<_>>()
        .join("\n")
}