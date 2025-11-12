pub fn pl(text: &[[u64; 26]; 26], reference: &[[u64; 26]; 26], a: f64) -> f64 {
    let mut sum = 0f64;

    for (tt, rr) in text.iter().zip(reference) {
        for (t, r) in tt.iter().zip(rr) {
            sum += *t as f64 * (*r as f64 + a).ln()
        }
    }

    sum
}