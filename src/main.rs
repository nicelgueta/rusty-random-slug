use rustyrs::{combinations, random_slugs, GeneralException};

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let num_words: i32 = args[1]
        .parse()
        .expect("Must provide length of slug in words");
    let num_outputs: Option<i32> = Some(match args.len() {
        3 => args[2].parse().unwrap(),
        _ => 1,
    });
    let phrases = random_slugs(num_words, num_outputs);
    match phrases {
        Ok(ps) => for p in ps {
            println!("{}", p)
        },
        Err(e) => match e{
            GeneralException::NoMoreUniqueCombinations => print!(
                "Requested more outputs than possible unique combinations. Max for {}-word slugs: {}\n",
                num_words, combinations(num_words).expect("Invalid number of words - must be between 1 and 5")
            ),
            e => println!("{}", String::from(e))
        }
    };
}