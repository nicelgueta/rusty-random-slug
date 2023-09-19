
use std::env;
use serde::{Deserialize};
use rand::seq::SliceRandom;

#[derive(Deserialize)]
struct WordFile {
    words: Vec<String>,
}

// bundle the files into the executable
static NOUN_FILE: &'static [u8] = include_bytes!("./data/nouns.json");
static ADJ_FILE: &'static [u8] = include_bytes!("./data/adjs.json");

fn main() { 
    let args: Vec<String> = env::args().collect();
    let num_words: u32 = args[1].parse().unwrap();
    if num_words < 2 {
        println!("Cannot work with < 2 words!");
    } else if num_words > 5 {
        println!("Cannot work with > 5 words");
    } else {
        let num_outputs: u32 = args[2].parse().unwrap();
        let phrases: Vec<String> = create_phrases(num_words, num_outputs);
        for i in 0..phrases.len() {
            println!("{}", phrases[i]);
        }
    }
}

fn get_words(word_file: &[u8]) -> Vec<String> {
    let contents: &str = std::str::from_utf8(word_file).unwrap();
    let json: WordFile = serde_json::from_str(&contents)
        .unwrap_or_else(|_| panic!("Cannot parse JSON file"));
    json.words
}

fn create_phrases(num_words: u32, num_outputs: u32) -> Vec<String> {
    let adjs: Vec<String> = get_words(ADJ_FILE);
    let nouns: Vec<String> = get_words(NOUN_FILE);
    let mut phrases: Vec<String> = Vec::new();
    for _ in 0..num_outputs {
        let phrase = create_phrase(&adjs, &nouns, num_words);
        phrases.push(phrase);
    }
    phrases
}

fn choose_word(vect: &Vec<String>) -> String {
    let word: String = vect.choose(&mut rand::thread_rng())
        .unwrap_or(&String::from("default"))
        .clone();
    word
}

fn create_phrase(adjs: &Vec<String>, nouns: &Vec<String>, num_words: u32) -> String {
    let noun: String = choose_word(nouns);
    if num_words == 2 {
        let adj: String = choose_word(adjs);
        let phrase: String = format!("{}-{}", adj, noun);
        phrase
    } else  {
        let adj1: String = choose_word(adjs);
        let adj2: String = choose_word(adjs);
        let phrase: String = if num_words == 3 {
            format!("{}-{}-{}", adj1, adj2, noun)
        } else if num_words == 4 {
            let noun2: String = choose_word(nouns);
            format!("{}-{}-{}-of-{}", adj1, adj2, noun, noun2)
        } else {
            let adj3: String = choose_word(adjs);
            let noun2: String = choose_word(nouns);
            format!("{}-{}-{}-of-{}-{}", adj1, adj2, noun, adj3, noun2)
        };
        phrase
    } 
}
