use rand::Rng;
use std::char;

fn generate_password(charset: &str, len: u32) -> String {
    let mut result = String::new();
    let mut rng = rand::thread_rng();
    for _ in 0..len {
        let idx = rng.gen_range(0..charset.len());
        result.push(char::from_u32(charset.as_bytes()[idx] as u32).expect(""));
    }
    return result;
}

fn main() {
    println!("{}", generate_password("qwertyuiopasdfghjklzxcvbnm", 10));
}