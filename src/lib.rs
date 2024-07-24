
pub use core::random_slugs;

#[cfg(feature = "wasm")]
mod wasm {
    use crate::core::{
        random_slugs as _random_slugs,
        // combinations as _combinations
    };
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn random_slugs(word_length: i32, num_outputs: Option<i32>) -> Option<Vec<String>> { 
        match _random_slugs(word_length, num_outputs) {
            Ok(v) => Some(v),
            Err(_e) => None
        }
    }

    // TODO: fix the fact integers overflow in wasm
    // #[wasm_bindgen]
    // pub fn combinations(word_length: i32) -> Option<i64> {
    //     match _combinations(word_length) {
    //         Ok(v) => Some(v as i64),
    //         Err(_e) => None
    //     }
    // }
}

#[cfg(feature = "python")]
mod python {
    use pyo3::prelude::*;
    use pyo3::exceptions::PyValueError;

    use crate::core::{
        random_slugs as _random_slugs, 
        get_slug as _get_slug,
        combinations as _combinations
    };
    #[pyclass]
    pub struct SlugGenerator {
        word_length: i32
    }

    #[pymethods]
    impl SlugGenerator {
        #[new]
        fn new(word_length: i32) -> PyResult<Self> {
            if word_length < 1 || word_length > 5 {
                Err(PyValueError::new_err(
                    "word_length must be between 1 and 5"
                ))
            } else {
                Ok(Self {word_length})
            }
        }
        fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
            slf
        }

        fn __next__(slf: PyRef<'_, Self>) -> PyResult<String> {
            match _get_slug(slf.word_length) {
                Ok(slug) => Ok(slug), 
                Err(_e) => Err(PyValueError::new_err(
                    "word_length is not between 1 and 5. It is likely this 
                    was changed after the class had been instantiated. 
                    "
                ))
            }
        }
    }

    #[pyfunction]
    fn get_slug(word_length: i32) -> PyResult<String> {
        match _get_slug(word_length) {
            Ok(i) => Ok(i),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }
    
    #[pyfunction]
    fn combinations(word_length: i32) -> PyResult<usize> {
        match _combinations(word_length) {
            Ok(i) => Ok(i),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }

    #[pyfunction]
    fn random_slugs(word_length: i32, num_outputs: Option<i32>) -> PyResult<Vec<String>> {
        if 0 < word_length && word_length < 6 {
            Ok(_random_slugs(word_length, num_outputs).unwrap())
        } else {
            Err(PyValueError::new_err("Number of words must be between 1 an 5"))
        }
    }
    
    #[pymodule]
    fn rustyrs(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(random_slugs, m)?)?;
        m.add_function(wrap_pyfunction!(get_slug, m)?)?;
        m.add_function(wrap_pyfunction!(combinations, m)?)?;
        m.add_class::<SlugGenerator>()?;
        Ok(())
    }
}



mod core {
    use std::error::Error;
    use rand::seq::SliceRandom;

    // bundle the files into the executable
    static NOUN_FILE: &'static [u8] = include_bytes!("./data/nouns.txt");
    static ADJ_FILE: &'static [u8] = include_bytes!("./data/adjs.txt");
    
    pub fn random_slugs(word_length: i32, num_outputs: Option<i32>) -> Result<Vec<String>, Box<dyn Error>> { 
        let num_outputs_u = num_outputs.unwrap_or(1);
        create_phrases(word_length, num_outputs_u)
    }
    pub fn get_slug(word_length: i32) -> Result<String, Box<dyn Error>> {
        let adjs: Vec<String> = get_words(ADJ_FILE);
        let nouns: Vec<String> = get_words(NOUN_FILE);
        create_phrase(&adjs, &nouns, word_length)
    }
    
    pub fn combinations(word_length: i32) -> Result<usize, Box<dyn Error>> {
        let adjs: Vec<String> = get_words(ADJ_FILE);
        let nouns: Vec<String> = get_words(NOUN_FILE);
        match word_length {
            1 => Ok(nouns.len()),
            2 => Ok(adjs.len() * nouns.len()),
            3 => Ok( ( adjs.len().pow(2) ) * nouns.len()),
            4 => Ok( ( adjs.len().pow(2) ) * ( nouns.len().pow(2) )),
            5 => Ok( ( adjs.len().pow(3) ) * ( nouns.len().pow(2) )),
            n => Err(format!(
                "Only slugs of length 1 to 5 are supported. Tried: {}", n
            ).into())
        }
    }
    fn get_words(word_file: &[u8]) -> Vec<String> {
        let contents: &str = std::str::from_utf8(word_file).unwrap();
        let words = contents.split("\n").map(
            |s|s.to_string()
         ).collect();
        words
    }
    
    fn create_phrases(word_length: i32, num_outputs: i32) -> Result<Vec<String>, Box<dyn Error>> {
        let adjs: Vec<String> = get_words(ADJ_FILE);
        let nouns: Vec<String> = get_words(NOUN_FILE);
        let mut phrases: Vec<String> = Vec::new();
        for _ in 0..num_outputs {
            let phrase = create_phrase(&adjs, &nouns, word_length)?;
            phrases.push(phrase);
        }
        Ok(phrases)
    }
    
    fn choose_word(vect: &Vec<String>) -> String {
        let word: String = vect.choose(&mut rand::thread_rng())
            .unwrap_or(&String::from("default"))
            .clone();
        word
    }
    
    fn create_phrase(adjs: &Vec<String>, nouns: &Vec<String>, word_length: i32) -> Result<String, Box<dyn Error>> {
        let noun: String = choose_word(nouns);
        match word_length {
            1 => Ok(noun),
            2 => {
                let adj: String = choose_word(adjs);
                Ok(format!("{}-{}", adj, noun))
            },
            3 => {
                let adj1: String = choose_word(adjs);
                let adj2: String = choose_word(adjs);
                Ok(format!("{}-{}-{}", adj1, adj2, noun))
            },
            4 => {
                let adj1: String = choose_word(adjs);
                let adj2: String = choose_word(adjs);
                let noun2: String = choose_word(nouns);
                Ok(format!("{}-{}-{}-of-{}", adj1, adj2, noun, noun2))
            },
            5 => {
                let adj1: String = choose_word(adjs);
                let adj2: String = choose_word(adjs);
                let adj3: String = choose_word(adjs);
                let noun2: String = choose_word(nouns);
                Ok(format!("{}-{}-{}-of-{}-{}", adj1, adj2, noun, adj3, noun2))
            },
            n => Err(format!(
                "Only slugs of length 1 to 5 are supported. Tried: {}", n
            ).into())
        }
    }
}


#[cfg(test)]
mod tests {

    use super::core::{random_slugs, combinations};

    #[test]
    fn happy() {
        for i in 1..5 {
            assert!(random_slugs(i, Some(1)).unwrap().len() > 0);
        }
    }

    #[test]
    fn unhappy_high() {
        match random_slugs(6, Some(1)) {
            Ok(_v)   => assert!(false),
            Err(_e) => assert!(true)
        }
    }

    #[test]
    fn unhappy_low() {
        match random_slugs(0,Some(1)) {
            Ok(_v)   => assert!(false),
            Err(_e) => assert!(true)
        }
    }

    #[test]
    fn unhappy_negative() {
        match random_slugs(-1, Some(1)) {
            Ok(_v)   => assert!(false),
            Err(_e) => assert!(true)
        }
    }

    #[test]
    fn combinations_happy() {
        let mut combo = 0;
        for i in 1..5 {
            let val = combinations(i).unwrap();
            assert!(val > combo);
            combo+=val
        }
    }

    #[test]
    fn combinations_unhappy_high() {
        match combinations(6) {
            Ok(_v)   => assert!(false),
            Err(_e) => assert!(true)
        }
    }

    #[test]
    fn combinations_unhappy_low() {
        match combinations(0) {
            Ok(_v)   => assert!(false),
            Err(_e) => assert!(true)
        }
    }

    #[test]
    fn combinations_unhappy_negative() {
        match combinations(-1) {
            Ok(_v)   => assert!(false),
            Err(_e) => assert!(true)
        }
    }
}
