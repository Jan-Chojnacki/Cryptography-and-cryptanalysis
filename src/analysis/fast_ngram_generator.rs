pub fn fast_ngram_generator(input: &Vec<u8>) -> [f32; 676] {
    let mut ngram: [f32; 676] = [0.0; 676];

    input.windows(2).for_each(|w| unsafe {
        *ngram.get_unchecked_mut((w[0] - b'A') as usize * 26 + (w[1] - b'A') as usize) += 1.0;
    });

    ngram
}
