use crate::operating_mode::OperatingMode;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn input_parser(input: File) -> String {
    let reader = BufReader::new(input);
    let mut buf: Vec<String> = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let filtered_string: String = line.chars().filter(|c| c.is_alphabetic()).collect();
            buf.push(filtered_string.to_uppercase())
        }
    }

    buf.join("")
}

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
                OperatingMode::ENCRYPTION => {
                    let key = parts[0].chars().next().unwrap();
                    let value = parts[1].chars().next().unwrap();
                    map.insert(key, value);
                }
                OperatingMode::DECRYPTION => {
                    let key = parts[1].chars().next().unwrap();
                    let value = parts[0].chars().next().unwrap();
                    map.insert(key, value);
                }
                _ => {}
            }
        }
    }

    map
}
