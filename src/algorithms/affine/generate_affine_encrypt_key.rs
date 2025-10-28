use std::collections::HashMap;
use gcd::euclid_u32;

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
        let y = (a * x + b).rem_euclid(M);
        let c = (b'A' + y as u8) as char;
        map.insert(p, c);
    }
    map
}