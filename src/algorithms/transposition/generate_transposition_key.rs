use std::collections::HashMap;

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