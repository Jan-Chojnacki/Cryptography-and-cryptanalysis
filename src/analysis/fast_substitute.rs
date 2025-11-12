pub fn fast_substitute(input: &str, key: &[char; 26]) -> String {
    input
        .chars()
        .map(|c| key[(c as u8 - b'A') as usize])
        .collect()
}
