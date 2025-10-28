use std::collections::HashMap;

pub fn substitute(input: &str, key: HashMap<char, char>) -> String {
    input.chars().map(|x| key.get(&x).unwrap()).collect()
}