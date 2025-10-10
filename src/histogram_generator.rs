use std::collections::HashMap;

pub fn histogram_generator(ngram: Vec<String>) -> HashMap<String, u64> {
    ngram
        .iter()
        .fold(HashMap::new(), |mut acc, gram| {
            *acc.entry(gram.clone()).or_insert(0) += 1;
            acc
        })
}