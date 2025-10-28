use bare_metal_modulo::{MNum, ModNum};
use std::collections::HashMap;

/// Generuje tablicę podstawień deszyfrujących dla szyfru afinicznego.
///
/// Oblicza odwrotność modularną parametru `a` i tworzy mapowanie od szyfrogramu do
/// tekstu jawnego dla wszystkich wielkich liter alfabetu łacińskiego.
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
