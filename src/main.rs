use rand::Rng;
use rand::prelude::ThreadRng;
use std::char;

struct PassGen {
    charset: String,
    rng: ThreadRng,
}

impl PassGen {
    pub fn new(charset: &str) -> Self {
        Self {
            charset: String::from(charset),
            rng: rand::thread_rng()
        }
    }
    pub fn gen(&mut self, len: u32) -> String {
        let mut result = String::new();
        for _ in 0..len {
            let idx = self.rng.gen_range(0..self.charset.len());
            result.push(
                char::from_u32(self.charset.as_bytes()[idx] as u32).expect("")
            );
        }
        return result;
    }
}

fn main() {
    let mut pgn = PassGen::new("qwertyuiopasdfghjklzxcvbnm");
    println!("{}", pgn.gen(10));
}