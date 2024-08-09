
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
            if word_length < 1 && word_length > 5 {
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
        create_phrases(word_length as usize, num_outputs_u)
    }
    pub fn get_slug(word_length: i32) -> Result<String, Box<dyn Error>> {
        let adjs: Vec<String> = get_words(ADJ_FILE);
        let nouns: Vec<String> = get_words(NOUN_FILE);
        let mut ws = WordSelector::new(adjs, nouns, word_length as usize)?;
        ws.choose()
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
    
    fn create_phrases(word_length: usize, num_outputs: i32) -> Result<Vec<String>, Box<dyn Error>> {
        let mut rng = rand::thread_rng();
        let mut adjs: Vec<String> = get_words(ADJ_FILE);
        adjs.shuffle(&mut rng);
        let mut nouns: Vec<String> = get_words(NOUN_FILE);
        nouns.shuffle(&mut rng);
        let mut ws = WordSelector::new(adjs, nouns, word_length)?;
        let mut words = Vec::new();
        for _i in 0..num_outputs {
            words.push(ws.choose()?)
        };
        Ok(words)
    }
    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            return a;
        }
        gcd(b, a % b)
    }
    
    fn lcm(a: usize, b: usize) -> usize {
        (a * b) / gcd(a, b)
    }
    
    /// This special class is designed to ensure uniqueness when generating random names.
    /// 
    struct WordSelector {
        adjs: Vec<String>,
        nouns: Vec<String>,
        adj_i: usize, 
        noun_i: usize,
        word_len: usize,
        increment_pos: bool,
        lcm: usize,
        total_combinations: usize,
        its_completed: usize
    }
    impl WordSelector {
        fn new(adjs: Vec<String>, nouns: Vec<String>, word_len: usize) -> Result<Self, Box<dyn Error>> {
            let lcm: usize = lcm(adjs.len(), nouns.len());
            Ok(Self { 
                adjs, 
                nouns, 
                adj_i: 0, 
                noun_i: 0, 
                word_len,
                increment_pos: true,
                lcm,
                total_combinations: combinations(word_len as i32)?,
                its_completed: 0
            })
        }
        pub fn choose(&mut self) -> Result<String, Box<dyn Error>>{
            match self.word_len{
                // 1 => self.choose_1(),
                2 => self.choose_2(),
                // 3 => self.choose_3(),
                // 4 => self.choose_4(),
                // 5 => self.choose_5(),
                n => Err(format!(
                    "Only slugs of length 1 to 5 are supported. Tried: {}", n
                ).into())
            }
        }
        fn choose_2(&mut self) -> Result<String, Box<dyn Error>> {
            let phrase = format!("{}-{}", self.adjs[self.adj_i], self.nouns[self.noun_i]);
            if self.its_completed == self.total_combinations {
                return Err("All unique combinations reached".into())
            };
            if self.its_completed == self.lcm {
                self.increment_pos = false;
            };
            if self.adj_i + 1 < self.adjs.len(){
                self.adj_i+=1;
            } else {
                self.adj_i = 0;
            }
            if self.increment_pos {
                self.noun_i = if self.noun_i + 1 < self.nouns.len(){
                    self.noun_i + 1
                } else {
                    0
                }
            } else {
                self.noun_i = if self.noun_i == 0 {
                    self.nouns.len() - 1
                } else {
                    self.noun_i - 1
                }
            };
            self.its_completed+=1;
            Ok(phrase)            
        }
        
    }    
}


#[cfg(test)]
mod tests {

    use std::collections::HashSet;

    use super::core::{random_slugs, combinations};

    #[test]
    fn happy_2() {
        assert!(random_slugs(2, Some(1)).unwrap().len() > 0);
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

    #[test]
    fn happy_2_all_unique_half() {
        let combos = combinations(2).unwrap() / 2;
        let slugs = random_slugs(2, Some(combos as i32)).expect(
            "unable to create 2 word slugs for all possible combinations"
        );
        assert!(slugs.len() == combos);
        let mut hs = HashSet::new();
        dbg!(&slugs[..10]);
        for slug in slugs {
            hs.insert(slug);
        };
        assert_eq!(hs.len(), combos)
    }

    #[test]
    fn happy_2_all_unique_all() {
        let possible_combos = combinations(2).unwrap();
        let slugs = random_slugs(2, Some(possible_combos as i32)).expect(
            "unable to create 2 word slugs for all possible combinations"
        );
        assert!(slugs.len() == possible_combos);
        let mut hs = HashSet::new();
        dbg!(&slugs[..10]);
        for slug in slugs {
            hs.insert(slug);
        };
        assert_eq!(hs.len(), possible_combos)
    }
}