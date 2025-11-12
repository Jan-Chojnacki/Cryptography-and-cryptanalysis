pub fn fast_ngram_generator(input: &str) -> [[u64; 26]; 26] {
    let mut ngram: [[u64; 26]; 26] = [[0; 26]; 26];

    input
        .as_bytes()
        .windows(2)
        .for_each(|w| {
            ngram[(w[0]-b'A') as usize][(w[1]-b'A') as usize] += 1;
        });

    ngram
}