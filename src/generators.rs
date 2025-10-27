use bare_metal_modulo::{MNum, ModNum};
use gcd::euclid_u32;
use std::collections::HashMap;

/// Creates a rolling window of n-grams from the provided input text.
pub fn ngram_generator(input: &str, ngram_size: u8) -> Vec<String> {
    // Slide over the bytes to capture every n-length subsequence.
    input
        .as_bytes()
        .windows(ngram_size as usize)
        .map(|w| String::from_utf8_lossy(w).to_string())
        .collect()
}

/// Aggregates a collection of n-grams into a histogram sorted by frequency.
pub fn histogram_generator(ngram: Vec<String>) -> HashMap<String, u64> {
    ngram
        .iter()
        .fold(HashMap::<String, u64>::new(), |mut acc, gram| {
            *acc.entry(gram.clone()).or_insert(0) += 1;
            acc
        })
}

pub fn generate_transposition_key(n: i16) -> HashMap<char, char> {
    let mut key = HashMap::with_capacity(26);
    let shift: u8 = ((n + 26) % 26) as u8;

    for i in 0..26 {
        let from = (b'A' + i) as char;
        let to = (b'A' + ((i + shift) % 26)) as char;
        key.insert(from, to);
    }

    key
}

pub fn generate_affine_encrypt_key(a: u32, b: u32) -> HashMap<char, char> {
    const M: u32 = 26;

    assert_eq!(
        euclid_u32(a, M),
        1,
        "Parameter `a` must be coprime with 26 (e.g., 1,3,5,7,9,11,15,17,19,21,23,25)."
    );

    let b = b.rem_euclid(M);

    let mut map = HashMap::with_capacity(26);
    for x in 0..M {
        let p = (b'A' + x as u8) as char;
        let c_val = (a * x + b).rem_euclid(M);
        let c = (b'A' + c_val as u8) as char;
        map.insert(p, c);
    }
    map
}

pub fn generate_affine_decrypt_key(a: u32, b: u32) -> HashMap<char, char> {
    const M: i32 = 26;

    let a = (a % 26) as i32;
    let b = (b % 26) as i32;

    let a_inv = ModNum::new(a, M)
        .inverse()
        .map(|inv| inv.a())
        .expect("Parameter `a` must be coprime with 26; no modular inverse");

    let mut map = HashMap::with_capacity(26);
    for y in 0..M {
        let c = (b'A' + y as u8) as char;
        let x = (a_inv * (y - b)).rem_euclid(M);
        let p = (b'A' + x as u8) as char;
        map.insert(c, p);
    }
    map
}
