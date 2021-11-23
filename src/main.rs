use rand::Rng;
use rand::prelude::ThreadRng;
use std::char;
use std::process::exit;
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

const LOWER_CASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPER_CASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS:    &str = "0123456789";
const SPECIAL:    &str = "~!@#$%^&*_+-=[]{}()<>`\\|;:'\",./?";

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
        .arg(
            Arg::with_name("chars")
                .short("c")
                .long("chars")
                .help("Which addional chars should be in password.")
                .takes_value(true)
                .value_name("chars")
        )
        .arg(
            Arg::with_name("all")
                .short("a")
                .long("all")
                .help("All predefiend characters.")
        )
        .arg(
            Arg::with_name("alfa")
                .short("A")
                .long("alfa")
                .help("All laters lower and upper case.")
        )
        .arg(
            Arg::with_name("upper")
                .short("u")
                .long("upper")
                .help("All laters upper case.")
        )
        .arg(
            Arg::with_name("lower")
                .short("L")
                .long("lower")
                .help("All laters lower case.")
        )
        .arg(
            Arg::with_name("numbers")
                .short("N")
                .long("numbers")
                .help("All of numbers")
        )
        .arg(
            Arg::with_name("special")
                .short("s")
                .long("special")
                .help("Special charakters like `!`.")
        )
        .arg(
            Arg::with_name("space")
                .short("S")
                .long("space")
                .help("Space charakter.")
        )
        .get_matches()
}

fn main() {
    let args = arg_parser();
    let mut charset = String::new();
    if args.is_present("space") {
        charset.push_str(" ");
    }
    if args.is_present("all") || args.is_present("upper") || args.is_present("alfa") {
        charset.push_str(UPPER_CASE);
    }
    if args.is_present("all") || args.is_present("lower") || args.is_present("alfa") {
        charset.push_str(LOWER_CASE);
    }
    if args.is_present("all") || args.is_present("numbers") {
        charset.push_str(NUMBERS);
    }
    if args.is_present("all") || args.is_present("special") {
        charset.push_str(SPECIAL);
    }
    if let Some(chars) = args.value_of("chars") {
        charset.push_str(chars);
    }
    // TODO: Clean unwraps.
    let length: u32 = args.value_of("length").unwrap().parse().unwrap();
    if length == 0 {
        eprintln!("password must have non zero legth.");
        exit(1);
    }
    let count:  u32 = args.value_of("count").unwrap().parse().unwrap();
    if charset.len() == 0 {
        eprintln!("empty charset.");
        exit(1);
    }
    let mut pgn = PassGen::new(&charset);
    for _ in 0..count {
        println!("{}", pgn.gen(length));
    }
}