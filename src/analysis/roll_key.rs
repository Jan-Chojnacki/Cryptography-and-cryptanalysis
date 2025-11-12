use rand::seq::SliceRandom;
use std::collections::HashMap;

pub fn roll_key(key: &[char; 26]) -> [char; 26] {
    let mut new_key = key.clone();

    new_key.shuffle(&mut rand::thread_rng());
    let a = new_key[0];
    let b = new_key[1];

    new_key[1] = a;
    new_key[0] = b;

    new_key
}
