use crate::operating_mode::OperatingMode;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

/// Reads the plaintext file and returns the concatenation of all alphabetic characters in uppercase.
pub fn input_parser(input: File) -> String {
    let reader = BufReader::new(input);
    let mut buf: Vec<String> = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            // Keep only ASCII alphabetic characters and normalise their case.
            let filtered_string: String =
                line.chars().filter(|c| c.is_ascii_alphabetic()).collect();
            buf.push(filtered_string.to_uppercase())
        }
    }

    buf.join("")
}

/// Parses the substitution key file into a mapping tailored to the requested operating mode.
pub fn key_parser(key: File, mode: &OperatingMode) -> HashMap<char, char> {
    let mut map: HashMap<char, char> = HashMap::new();
    let reader = BufReader::new(key);

    for line in reader.lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 2 {
                panic!("Invalid key format.")
            }
            match mode {
                OperatingMode::Encryption => {
                    // In encryption mode the file lists plaintext to ciphertext pairs.
                    let key = parts[0].chars().next().unwrap();
                    let value = parts[1].chars().next().unwrap();
                    map.insert(key, value);
                }
                OperatingMode::Decryption => {
                    // In decryption mode the mapping is inverted to translate ciphertext back to plaintext.
                    let key = parts[1].chars().next().unwrap();
                    let value = parts[0].chars().next().unwrap();
                    map.insert(key, value);
                }
                _ => {}
            }
        }
    }

    // Validate that all letters are represented once both as keys and values.
    let key_test: HashSet<char> = map.iter().map(|(&k, _)| k).collect();
    let value_test: HashSet<char> = map.iter().map(|(_, &v)| v).collect();

    if key_test.len() != 26 || value_test.len() != 26 {
        panic!("Invalid key values.")
    }

    map
}

/// Reads an n-gram frequency file and converts the counts into probabilities.
pub fn ngram_parser(ngram: File, n: u8) -> Vec<(String, f64)> {
    let mut map: Vec<(String, u64)> = Vec::new();
    let reader = BufReader::new(ngram);

    let mut sum: u64 = 0;

    for line in reader.lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 2 {
                panic!("Invalid ngram format.")
            }
            let key = parts[0].to_string();
            let value = u64::from_str(parts[1]).unwrap();
            if key.len() != n as usize {
                dbg!(key);
                panic!("Invalid ngram format.")
            }

            // Track the raw occurrence count before normalising to probabilities.
            map.push((key, value));
            sum += value;
        }
    }

    // Convert each raw count to a probability using the total number of observations.
    map.iter()
        .map(|(k, v)| (k.clone(), *v as f64 / sum as f64))
        .collect()
}
