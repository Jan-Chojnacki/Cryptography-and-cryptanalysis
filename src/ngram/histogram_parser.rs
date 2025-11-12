use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn histogram_parser(ngram: File, n: u8) -> HashMap<String, u64> {
    let mut map: HashMap<String, u64> = HashMap::new();
    let reader = BufReader::new(ngram);

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

            map.insert(key, value);
        }
    }

    map
}