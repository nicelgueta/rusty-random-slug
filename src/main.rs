use rustyrs::{combinations, random_slugs, GeneralException};


use clap::Parser;

/// Simple CLI to generate unique slugs
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct RustyrsArgs{

    /// Number of words in the slug. Between 1 and 5 inclusive.
    #[arg(short, long, default_value_t = 2)]
    num_words: i32,

    /// Number of slugs to generate. Note that
    #[arg(short = 's', long, default_value_t = 1)]
    num_slugs: i32
}

fn main() -> Result<(), GeneralException> {
    let args = RustyrsArgs::parse();

    let phrases = random_slugs(
        args.num_words, Some(args.num_slugs)
);
    match phrases {
        Ok(ps) => {
            for p in ps {
                println!("{}", p)
            };
            Ok(())
        },
        Err(e) => {
            match e.clone() {
                GeneralException::NoMoreUniqueCombinations => print!(
                    "Requested more outputs than possible unique combinations. Max for {}-word slugs: {}\n",
                    args.num_words, combinations(args.num_words).expect("Invalid number of words - must be between 1 and 5")
                ),
                er => println!("{}", String::from(er))
            };
            Err(e)
        }
    }
}