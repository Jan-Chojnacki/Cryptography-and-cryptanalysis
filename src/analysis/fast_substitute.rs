pub fn fast_substitute(out: &mut Vec<u8>, input: &str, key: &[u8; 26]) {
    out.resize(input.len(), 0);
    for (dst, &b) in out.iter_mut().zip(input.as_bytes()) {
        *dst = key[(b - b'A') as usize];
    }
}
