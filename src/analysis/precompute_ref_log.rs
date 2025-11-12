use fast_math::log2_raw;

pub fn precompute_ref_log(reference: &[[f32; 26]; 26], a: f32) -> [[f32; 26]; 26] {
    let mut out = [[0.0f32; 26]; 26];
    for i in 0..26 {
        for j in 0..26 {
            out[i][j] = log2_raw(reference[i][j] + a);
        }
    }
    out
}