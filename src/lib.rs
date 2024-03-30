use wasm_bindgen::prelude::*;
use serde::Deserialize;
use rand::seq::SliceRandom;

#[derive(Deserialize)]
struct WordFile {
    words: Vec<String>,
}

// bundle the files into the executable
static NOUN_FILE: &'static [u8] = include_bytes!("./data/nouns.json");
static ADJ_FILE: &'static [u8] = include_bytes!("./data/adjs.json");


#[wasm_bindgen]
pub fn random_slugs(num_words: u32, num_outputs: Option<u32>) -> Option<Vec<String>> { 
    let num_outputs_u = num_outputs.unwrap_or(1);
    if num_words < 1 {
        println!("Cannot work with < 1 word!");
        None
    } else if num_words > 5 {
        println!("Cannot work with > 5 words");
        None
    } else {
        let phrases: Vec<String> = create_phrases(num_words, num_outputs_u);
        Some(phrases)
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
    match num_words {
        1 => noun,
        2 => {
            let adj: String = choose_word(adjs);
            format!("{}-{}", adj, noun)
        },
        3 => {
            let adj1: String = choose_word(adjs);
            let adj2: String = choose_word(adjs);
            format!("{}-{}-{}", adj1, adj2, noun)
        },
        4 => {
            let adj1: String = choose_word(adjs);
            let adj2: String = choose_word(adjs);
            let noun2: String = choose_word(nouns);
            format!("{}-{}-{}-of-{}", adj1, adj2, noun, noun2)
        },
        5 => {
            let adj1: String = choose_word(adjs);
            let adj2: String = choose_word(adjs);
            let adj3: String = choose_word(adjs);
            let noun2: String = choose_word(nouns);
            format!("{}-{}-{}-of-{}-{}", adj1, adj2, noun, adj3, noun2)
        },
        n => panic!("{}", format!("Cannot process using {} words", n))
    }
}

#[cfg(test)]
mod tests {

    use super::random_slugs;

    #[test]
    fn happy() {
        for i in 1..5 {
            assert!(random_slugs(i, Some(1)).unwrap().len() > 0);
        }
    }

    #[test]
    fn unhappy_high() {
        match random_slugs(6, Some(1)) {
            Some(_v)   => assert!(false),
            None => assert!(true)
        }
    }

    #[test]
    fn unhappy_low() {
        match random_slugs(0,Some(1)) {
            Some(_v)   => assert!(false),
            None => assert!(true)
        }
    }
}