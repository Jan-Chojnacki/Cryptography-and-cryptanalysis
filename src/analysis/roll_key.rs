use rand::rngs::ThreadRng;
use rand::Rng;

pub fn roll_key(key: &[u8; 26], rng: &mut ThreadRng) -> [u8; 26] {
    let mut new_key = key.clone();

    let a = rng.gen_range(0..26);
    let mut b = rng.gen_range(0..25);
    if a == b {
        b += 1
    }

    new_key.swap(a,b);
    new_key
}
