use std::collections::HashMap;

pub fn pl(text: &HashMap<String, u64>, reference: &HashMap<String, u64>, a: f64) -> f64 {
    let mut sum = 0.0f64;
    for (k, v) in text {
        match reference.get(k) {
            None => {
                sum += a;
            }
            Some(rv) => {
                sum += *v as f64 * (*rv as f64 + a).log10()
            }
        }
    }

    sum
}