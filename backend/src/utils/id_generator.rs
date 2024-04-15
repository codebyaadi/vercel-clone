use rand::seq::IteratorRandom;
use rand::thread_rng;

const MAX_LEN: usize = 5;

pub fn generate() -> String {
    let subset = "123456789qwertyuiopasdfghjklzxcvbnm";
    let mut rng = thread_rng();
    let generated_string: String = (0..MAX_LEN)
        .map(|_| subset.chars().choose(&mut rng).unwrap())
        .collect();
    generated_string
}