use rand::distributions::{Distribution, WeightedIndex};
use rand::RngCore;

const NUM_OF_LETTERS: usize = 26;

pub struct Model {
    chars: [[usize; NUM_OF_LETTERS + 1]; NUM_OF_LETTERS + 1],
}

impl Model {
    pub fn new() -> Model {
        let chars = include!(concat!(env!("OUT_DIR"), "/prebuilt_model.rs"));
        Model { chars }
    }

    pub fn generate<Rng: RngCore>(&self, rng: &mut Rng, len: usize) -> String {
        let mut result = String::with_capacity(len);
        let mut prefix = NUM_OF_LETTERS;
        for _ in 0..len {
            let chars = &self.chars[prefix];
            let c = select_one_char(rng, &chars);
            result.push(c as char);
            prefix = (c - b'a') as usize;
        }
        result
    }
}

fn select_one_char<Rng: RngCore>(rng: &mut Rng, freq: &[usize; NUM_OF_LETTERS + 1]) -> u8 {
    let dist = WeightedIndex::new(freq).expect("invalid frequency");
    b'a' + (dist.sample(rng) as u8)
}
