use std::collections::HashMap;

pub fn x2test(ngram: &HashMap<String, u64>, ngram_ref: &HashMap<String, f64>, critical: f64) -> Result<(), f64> {
    let mut x2: f64 = 0.0;

    for (k, v) in ngram {
        if let Some(e) = ngram_ref.get(k) {
            x2 += (*v as f64 - e).powi(2) / e;
        }
    }

    match x2 <= critical {
        true => { Ok(()) }
        false => { Err(x2) }
    }
}