pub fn pl_pre_log(text: &[[f32; 26]; 26], ref_log: &[[f32; 26]; 26]) -> f32 {
    let mut sum: f32 = 0.0;

    //TODO do poprawy
    for (&t, &r) in text.iter().flatten().zip(ref_log.iter().flatten()) {
        sum = t.mul_add(r, sum);
    }

    sum
}
