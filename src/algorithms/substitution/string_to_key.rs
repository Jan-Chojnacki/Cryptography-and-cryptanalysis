use std::collections::HashMap;

pub fn string_to_key(key: &str) -> HashMap<char, char> {
    let alphabet: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let key: Vec<char> = key.chars().collect();

    assert_eq!(alphabet.len(), key.len());

    alphabet.into_iter().zip(key.into_iter()).collect()
}