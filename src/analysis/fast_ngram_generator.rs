pub fn fast_ngram_generator(input: &Vec<u8>) -> [[f32; 26]; 26] {
    let mut ngram: [[f32; 26]; 26] = [[0.0; 26]; 26];

    input
        .windows(2)
        .for_each(|w| {
            ngram[(w[0]-b'A') as usize][(w[1]-b'A') as usize] += 1.0;
        });

    ngram
}