use std::collections::HashMap;
use statrs::distribution::{ChiSquared, ContinuousCDF};

pub fn x2test(ngram: &HashMap<String, u64>, ngram_ref: &HashMap<String, f64>, df: f64, p: f64) -> bool {
    let n: u64 = ngram.values().sum();
    let mut x2: f64 = 0.0;

    for (k, v) in ngram {
        if let Some(rv) = ngram_ref.get(k) {
            let e = rv * n as f64;
            x2 += (*v as f64 - e).powi(2) / e;
        }
    }

    let chi = ChiSquared::new(df).expect("invalid degrees of freedom");
    let critical = chi.inverse_cdf(p);

    x2 <= critical
}