use fast_math::log2;

pub fn pl(text: &[[f32; 26]; 26], reference: &[[f32; 26]; 26], a: f32) -> f32 {
    let mut sum: f32 = 0.0;

    for (tt, rr) in text.iter().zip(reference) {
        for (t, r) in tt.iter().zip(rr) {
            sum += *t * log2(*r + a)
        }
    }

    sum
}
