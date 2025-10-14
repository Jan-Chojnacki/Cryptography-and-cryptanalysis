use std::collections::HashMap;

pub fn ngram_generator(input: &str, ngram_size: u8) -> Vec<String> {
    input
        .as_bytes()
        .windows(ngram_size as usize)
        .map(|w| String::from_utf8_lossy(w).to_string())
        .collect()
}

pub fn histogram_generator(ngram: Vec<String>) -> Vec<(String, u64)> {
    let mut res = ngram.iter().fold(HashMap::new(), |mut acc, gram| {
        *acc.entry(gram.clone()).or_insert(0) += 1;
        acc
    }).iter().fold(Vec::new(), |mut acc, (k, v)| {
        acc.push((k.clone(), *v));
        acc
    });

    res.sort_by_key(|&(_, v)| v);
    res.reverse();

    res
}
