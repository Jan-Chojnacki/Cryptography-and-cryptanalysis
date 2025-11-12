use fast_math::log2_raw;

pub fn precompute_ref_log(reference: &[f32; 676], a: f32) -> [f32; 676] {
    let mut out = [0.0; 676];
    for i in 0..26 {
        for j in 0..26 {
            out[i * 26 + j] = log2_raw(reference[i * 26 + j] + a);
        }
    }
    out
}
