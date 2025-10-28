use std::collections::HashMap;

/// Przeprowadza test chi-kwadrat dla rozkładu n-gramów względem rozkładu referencyjnego.
///
/// # Arguments
/// * `ngram` - Histogram zliczeń n-gramów uzyskany z badanego tekstu.
/// * `ngram_ref` - Referencyjny rozkład n-gramów znormalizowany do sumy 1.
/// * `critical` - Wartość krytyczna testu chi-kwadrat dla przyjętego poziomu istotności.
///
/// # Zwracana wartość
/// Zwraca `Ok(())`, jeśli hipoteza zerowa nie została odrzucona (statystyka mieści się
/// poniżej progu), w przeciwnym razie `Err` z wartością statystyki `chi^2`.
pub fn x2test(
    ngram: &HashMap<String, u64>,
    ngram_ref: &HashMap<String, f64>,
    critical: f64,
) -> Result<(), f64> {
    let mut x2: f64 = 0.0;

    for (k, v) in ngram {
        if let Some(e) = ngram_ref.get(k) {
            x2 += (*v as f64 - e).powi(2) / e;
        }
    }

    match x2 <= critical {
        true => Ok(()),
        false => Err(x2),
    }
}
