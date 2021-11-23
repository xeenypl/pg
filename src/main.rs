use rand::Rng;
use rand::prelude::ThreadRng;
use std::char;
use clap::App;
use clap::Arg;
use clap::ArgMatches;

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
                // TODO: Clean exepts.
                char::from_u32(self.charset.as_bytes()[idx] as u32)
                    .expect("Convertion fail")
            );
        }
        return result;
    }
}

fn arg_parser<'a>() -> ArgMatches<'a> {
    App::new("pgn - Command line password generator")
        .version("0.1.1")
        .author("Piotr `xeeny` Dudziński")
        .arg(
            Arg::with_name("length")
                .short("l")
                .long("length")
                .help("How long posword generate.")
                .takes_value(true)
                .value_name("length")
                .default_value("10")
        )
        .arg(
            Arg::with_name("count")
                .short("n")
                .long("count")
                .help("How many posword generate.")
                .takes_value(true)
                .value_name("count")
                .default_value("1")
        )
        .get_matches()
}

fn main() {
    let args = arg_parser();
    // TODO: Clean unwraps.
    let length: u32 = args.value_of("length").unwrap().parse().unwrap();
    let count:  u32 = args.value_of("count").unwrap().parse().unwrap();
    let mut pgn = PassGen::new("qwertyuiopasdfghjklzxcvbnm");
    for _ in 0..count {
        println!("{}", pgn.gen(length));
    }
}