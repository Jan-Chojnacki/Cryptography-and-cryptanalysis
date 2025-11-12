use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn fast_ngram_parser(ngram: File) -> [[f32; 26]; 26] {
    let mut map: [[f32; 26]; 26] = [[0.0; 26]; 26];
    let reader = BufReader::new(ngram);

    for line in reader.lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 2 {
                panic!("Invalid ngram format.")
            }
            let key = parts[0].as_bytes();
            let value = u64::from_str(parts[1]).unwrap();
            if key.len() != 2 {
                panic!("Invalid ngram format.")
            }

            map[(key[0] - b'A') as usize][(key[1] - b'A') as usize] = value as f32;
        }
    }

    map
}
