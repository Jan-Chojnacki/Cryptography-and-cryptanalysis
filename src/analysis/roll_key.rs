use rand::seq::SliceRandom;
use std::collections::HashMap;

pub fn roll_key(key: HashMap<char, char>) -> HashMap<char, char> {
    let mut new_key: Vec<(char, char)> = key.into_iter().collect();
    let mut values: Vec<usize> = (0..26).collect();
    values.shuffle(&mut rand::thread_rng());
    let tmp = new_key[values[0]];
    new_key[values[0]] = new_key[values[1]];
    new_key[values[1]] = tmp;
    new_key
        .iter()
        .fold(HashMap::<char, char>::new(), |mut acc, (k, v)| {
            acc.insert(*k, *v);
            acc
        })
}
