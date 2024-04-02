use std::env;
use rustyrs::random_slugs_f;

fn main() {
    let args: Vec<String> = env::args().collect();
    let num_words: u32 = args[1].parse().expect("Must provide length of slug in words");
    let num_outputs: Option<u32> = Some(match args.len() {
        3 => args[2].parse().unwrap(),
        _ => 1
    });
    let phrases = random_slugs_f(num_words, num_outputs);
    if let Some(ps) = phrases {
        for p in ps {
            println!("{}", p)
        }
    }
}