pub use core::*;

#[cfg(feature = "wasm")]
mod wasm {
    use crate::core::{
        random_slugs as _random_slugs,
        combinations as _combinations
    };
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn random_slugs(word_length: i32, num_outputs: Option<i32>) -> Option<Vec<String>> {
        match _random_slugs(word_length, num_outputs) {
            Ok(v) => Some(v),
            Err(_e) => None,
        }
    }

    // TODO: fix the fact integers overflow in wasm
    #[wasm_bindgen]
    pub fn combinations(word_length: i32) -> Option<i64> {
        match _combinations(word_length) {
            Ok(v) => Some(v as i64),
            Err(_e) => None
        }
    }
}

#[cfg(feature = "python")]
mod python {
    use pyo3::exceptions::{PyRuntimeError, PyValueError};
    use pyo3::prelude::*;
    use rand::seq::SliceRandom;

    use crate::core::{
        combinations as _combinations, 
        random_slugs as _random_slugs, 
        get_slug as _get_slug,
        GeneralException,
        WordSelector,
        get_words, 
        ADJ_FILE, 
        NOUN_FILE
    };

    #[pyclass]
    pub struct SlugGenerator {
        generator: WordSelector
    }

    #[pymethods]
    impl SlugGenerator {
        #[new]
        fn new(word_length: i32) -> PyResult<Self> {
            if word_length < 1 || word_length > 5 {
                Err(PyValueError::new_err("word_length must be between 1 and 5"))
            } else {
                let mut rng = rand::thread_rng();
                let mut adjs = get_words(ADJ_FILE);
                let mut nouns = get_words(NOUN_FILE);
                adjs.shuffle(&mut rng);
                nouns.shuffle(&mut rng);
                let generator = if let Ok(gen) = WordSelector::new(
                    adjs, nouns,
                    word_length as usize
                ) {
                    gen
                } else {
                    return Err(PyRuntimeError::new_err("Failure creating WordSelector object"))
                };
                Ok(Self {generator})
            }
        }
        fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
            slf
        }

        fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<String> {
            match slf.generator.choose() {
                Ok(slug) => Some(slug),
                // Err(e) => Err(PyValueError::new_err(
                //     e.to_string()
                // )),
                Err(e) => None
            }
        }
    }

    #[pyfunction]
    fn get_slug(word_length: i32) -> PyResult<String> {
        match _get_slug(word_length) {
            Ok(i) => Ok(i),
            Err(e) => Err(PyValueError::new_err(String::from(e))),
        }
    }

    #[pyfunction]
    fn combinations(word_length: i32) -> PyResult<usize> {
        match _combinations(word_length) {
            Ok(i) => Ok(i),
            Err(e) => Err(PyValueError::new_err(String::from(e))),
        }
    }

    #[pyfunction]
    fn random_slugs(word_length: i32, num_outputs: Option<i32>) -> PyResult<Vec<String>> {
        if 0 < word_length && word_length < 6 {
            match _random_slugs(word_length, num_outputs) {
                Ok(r) => Ok(r),
                Err(e) => match e {
                    GeneralException::NoMoreUniqueCombinations => Err(PyValueError::new_err(format!(
                        "Requested to generate more slugs than they are unique combinations. Max for {}-word slugs is: {}",
                        word_length, combinations(word_length).unwrap()
                    ))),
                    e => Err(PyValueError::new_err(String::from(e)))
                }
            }
        } else {
            Err(PyValueError::new_err(
                "Number of words must be between 1 an 5",
            ))
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
    use rand::seq::SliceRandom;
    
    #[derive(Debug)]
    pub enum GeneralException {
        NoMoreUniqueCombinations,
        InvalidWordLength(i32),
        Other(String)
    }
    impl From<GeneralException> for String {
        fn from(error: GeneralException) -> Self {
            match error {
                GeneralException::InvalidWordLength(got) => format!("Only slugs of length 1 to 5 are supported. Tried: {}", got),
                GeneralException::NoMoreUniqueCombinations => "Cannot generate any more unique combinations for this length in words".to_string(),
                GeneralException::Other(payload) => payload
            }        
        }
    }

    // bundle the files into the executable
    pub static NOUN_FILE: &'static [u8] = include_bytes!("./data/nouns.txt");
    pub static ADJ_FILE: &'static [u8] = include_bytes!("./data/adjs.txt");

    pub fn random_slugs(
        word_length: i32,
        num_outputs: Option<i32>,
    ) -> Result<Vec<String>, GeneralException> {
        let num_outputs_u = num_outputs.unwrap_or(1);
        let max_combos = combinations(word_length)?;
        if num_outputs_u as usize > max_combos {
            Err(GeneralException::NoMoreUniqueCombinations)
        } else {
            create_phrases(word_length as usize, num_outputs_u)
        }
    }
    pub fn get_slug(word_length: i32) -> Result<String, GeneralException> {
        let adjs: Vec<String> = get_words(ADJ_FILE);
        let nouns: Vec<String> = get_words(NOUN_FILE);
        let mut ws = WordSelector::new(adjs, nouns, word_length as usize)?;
        ws.choose()
    }

    pub fn combinations(word_length: i32) -> Result<usize, GeneralException> {
        let adjs: Vec<String> = get_words(ADJ_FILE);
        let nouns: Vec<String> = get_words(NOUN_FILE);
        match word_length {
            1 => Ok(nouns.len()),
            2 => Ok(adjs.len() * nouns.len()),
            3 => Ok((adjs.len().pow(2)) * nouns.len()),
            4 => Ok((adjs.len().pow(2)) * (nouns.len().pow(2))),
            5 => Ok((adjs.len().pow(3)) * (nouns.len().pow(2))),
            n => Err(GeneralException::InvalidWordLength(n)),
        }
    }
    pub fn get_words(word_file: &[u8]) -> Vec<String> {
        let contents: &str = std::str::from_utf8(word_file).unwrap();
        let words = contents.split("\n").map(|s| s.to_string()).collect();
        words
    }

    fn create_phrases(word_length: usize, num_outputs: i32) -> Result<Vec<String>, GeneralException> {
        let mut rng = rand::thread_rng();
        let mut adjs: Vec<String> = get_words(ADJ_FILE);
        adjs.shuffle(&mut rng);
        let mut nouns: Vec<String> = get_words(NOUN_FILE);
        nouns.shuffle(&mut rng);
        let mut ws = WordSelector::new(adjs, nouns, word_length)?;
        let mut words = Vec::new();
        for _i in 0..num_outputs {
            words.push(ws.choose()?)
        }
        Ok(words)
    }

    /// This special class is designed to ensure uniqueness when generating random names.
    /// It uses combinatoric logic to hold state between calls to .choose()
    pub struct WordSelector {
        adjs: Vec<String>,
        nouns: Vec<String>,
        selection_ptrs: Vec<Vec<usize>>,
        selection_i: usize,
        word_len: usize,
        total_combinations: usize,
        its_completed: usize,
    }
    impl WordSelector {
        pub fn new(
            adjs: Vec<String>,
            nouns: Vec<String>,
            word_len: usize,
        ) -> Result<Self, GeneralException> {
            let selection_ptrs = match word_len {
                1 => Vec::new(),
                2 => {
                    let mut ptrs = Vec::with_capacity(adjs.len());
                    let mut noun_i_ct = 0;
                    for _ in 0..adjs.len() {
                        ptrs.push(vec![noun_i_ct]);
                        noun_i_ct = if noun_i_ct == nouns.len() - 1 {
                            0
                        } else {
                            noun_i_ct + 1
                        };
                    }
                    ptrs
                }
                3 => {
                    let mut ptrs = Vec::with_capacity(adjs.len());
                    let mut noun_i = 0 as usize;
                    let mut adj_2_i = adjs.len() - 1;

                    for i in 0..adjs.len() {
                        ptrs.push(vec![adj_2_i, noun_i]);

                        noun_i = if noun_i == nouns.len() - 1 {
                            0
                        } else {
                            noun_i + 1
                        };
                        adj_2_i = adjs.len() - 1 - i;
                    }
                    ptrs
                }
                4 => {
                    let mut ptrs = Vec::with_capacity(adjs.len());
                    let mut noun_i = 0 as usize;
                    let mut adj_2_i = adjs.len() - 1;
                    let mut noun_2_i = nouns.len() - 1;

                    for i in 0..adjs.len() {
                        ptrs.push(vec![adj_2_i, noun_i, noun_2_i]);

                        noun_i = if noun_i == nouns.len() - 1 {
                            0
                        } else {
                            noun_i + 1
                        };
                        adj_2_i = adjs.len() - 1 - i;
                        noun_2_i = if noun_2_i == nouns.len() - 1 {
                            0
                        } else {
                            noun_2_i + 1
                        }
                    }
                    ptrs
                }
                5 => {
                    let mut ptrs = Vec::with_capacity(adjs.len());
                    let mut noun_i = 0 as usize;
                    let mut adj_2_i = adjs.len() - 1;
                    let mut adj_3_i = adjs.len() / 2;
                    let mut noun_2_i = nouns.len() - 1;

                    for i in 0..adjs.len() {
                        ptrs.push(vec![adj_2_i, noun_i, adj_3_i, noun_2_i]);

                        noun_i = if noun_i == nouns.len() - 1 {
                            0
                        } else {
                            noun_i + 1
                        };
                        adj_2_i = adjs.len() - 1 - i;
                        adj_3_i = if adj_3_i == adjs.len() - 1 {
                            0
                        } else {
                            adj_3_i + 1
                        };
                        noun_2_i = if noun_2_i == nouns.len() - 1 {
                            0
                        } else {
                            noun_2_i + 1
                        }
                    }
                    ptrs
                }
                n => return Err(GeneralException::InvalidWordLength(n as i32)),
            };
            Ok(Self {
                adjs,
                nouns,
                selection_ptrs,
                word_len,
                total_combinations: combinations(word_len as i32)?,
                its_completed: 0,
                selection_i: 0,
            })
        }
        pub fn choose(&mut self) -> Result<String, GeneralException> {
            if self.its_completed == self.total_combinations {
                return Err(GeneralException::NoMoreUniqueCombinations);
            }
            match self.word_len {
                1 => Ok(self.choose_1()),
                2 => Ok(self.choose_2()),
                3 => Ok(self.choose_3()),
                4 => Ok(self.choose_4()),
                5 => Ok(self.choose_5()),
                n => Err(GeneralException::InvalidWordLength(n as i32)),
            }
        }
        fn choose_1(&mut self) -> String {
            let phrase = self.nouns[self.selection_i].clone();
            self.selection_i += 1;
            phrase
        }
        /// Function to return a two word slug. The internal selection_map holds pointers
        /// to the adjective list as keys and a pointer to a noun as the value. For each
        /// iteration both pointers are incremented to ensure that each output does not contain
        /// similar word as the previous output. Pointers are wrapped when they go out of bounds
        /// to ensure all possible combinations can be generated.
        fn choose_2(&mut self) -> String {
            let noun_i = self.selection_ptrs[self.selection_i]
                .last()
                .unwrap()
                .clone();
            let phrase = format!("{}-{}", self.adjs[self.selection_i], self.nouns[noun_i]);
            let noun_ptr = self
                .selection_ptrs
                .get_mut(self.selection_i)
                .unwrap()
                .last_mut()
                .unwrap();

            *noun_ptr = if noun_i == self.nouns.len() - 1 {
                // ptr sent back to beginning of the noun array
                0
            } else {
                noun_i + 1
            };

            self.selection_i = if self.selection_i == self.selection_ptrs.len() - 1 {
                // reached the end of the adjective list so return to the beginning
                0
            } else {
                self.selection_i + 1
            };
            self.its_completed += 1;
            phrase
        }

        fn choose_3(&mut self) -> String {
            let adj_1_i = self.selection_i;
            let adj_2_i = self.selection_ptrs[self.selection_i][0];
            let noun_i = self.selection_ptrs[self.selection_i][1];

            let phrase = format!(
                "{}-{}-{}",
                self.adjs[adj_1_i], self.adjs[adj_2_i], self.nouns[noun_i]
            );

            let ptr_set = self
                .selection_ptrs
                .get_mut(self.selection_i)
                .expect("Unable to obtain mutable reference to index pointer set");

            if noun_i == self.nouns.len() - 1 {
                // reached end of iteration of nouns so decrement
                // the second adj_pointer
                ptr_set[0] = if adj_2_i == 0 {
                    self.adjs.len() - 1
                } else {
                    adj_2_i - 1
                };

                // reset noun pointer
                ptr_set[1] = 0
            } else {
                ptr_set[1] += 1
            }
            self.selection_i = if self.selection_i == self.selection_ptrs.len() - 1 {
                0
            } else {
                self.selection_i + 1
            };
            self.its_completed += 1;
            phrase
        }
        fn choose_4(&mut self) -> String {
            let adj_1_i = self.selection_i;
            let adj_2_i = self.selection_ptrs[self.selection_i][0];
            let noun_i = self.selection_ptrs[self.selection_i][1];
            let noun_2_i = self.selection_ptrs[self.selection_i][2];

            let phrase = format!(
                "{}-{}-of-{}-{}",
                self.adjs[adj_1_i], self.nouns[noun_i], self.adjs[adj_2_i], self.nouns[noun_2_i]
            );

            let ptr_set = self
                .selection_ptrs
                .get_mut(self.selection_i)
                .expect("Unable to obtain mutable reference to index pointer set");

            if noun_2_i == 0 {
                // reached end of iteration of 2nd noun so increment
                // the first noun pointer and reset noun 2 to top
                ptr_set[1] += 1;

                ptr_set[2] = self.nouns.len() - 1
            } else {
                ptr_set[2] -= 1;
            }

            if ptr_set[1] > self.nouns.len() - 1 {
                // decrement 2nd adjective on first noun iteration completion
                ptr_set[0] = if adj_2_i == 0 {
                    self.adjs.len() - 1
                } else {
                    adj_2_i - 1
                };

                // reset noun pointer
                ptr_set[1] = 0
            }
            self.selection_i = if self.selection_i == self.selection_ptrs.len() - 1 {
                0
            } else {
                self.selection_i + 1
            };
            self.its_completed += 1;
            phrase
        }
        fn choose_5(&mut self) -> String {
            let adj_1_i = self.selection_i;
            let adj_2_i = self.selection_ptrs[self.selection_i][0];
            let noun_i = self.selection_ptrs[self.selection_i][1];
            let adj_3_i = self.selection_ptrs[self.selection_i][2];
            let noun_2_i = self.selection_ptrs[self.selection_i][3];

            let phrase = format!(
                "{}-{}-{}-of-{}-{}",
                self.adjs[adj_1_i],
                self.adjs[adj_2_i],
                self.nouns[noun_i],
                self.adjs[adj_3_i],
                self.nouns[noun_2_i]
            );

            let ptr_set = self
                .selection_ptrs
                .get_mut(self.selection_i)
                .expect("Unable to obtain mutable reference to index pointer set");

            if ptr_set[3] == 0 {
                // reached end of iteration of 2nd noun so increment
                // the third adj pointer and reset noun 2 to top
                ptr_set[2] += 1;

                ptr_set[3] = self.nouns.len() - 1
            } else {
                ptr_set[3] -= 1;
            }

            if ptr_set[2] >= self.adjs.len() {
                // increment first noun on third adj it completion
                ptr_set[1] += 1;

                // reset third adj pointer
                ptr_set[2] = 0;
            }

            if ptr_set[1] >= self.nouns.len() {
                // decrement second adj on first noun it comp
                ptr_set[0] = if ptr_set[0] == 0 {
                    self.adjs.len() - 1
                } else {
                    ptr_set[0] - 1
                };
                ptr_set[1] = 0;
            }

            self.selection_i = if self.selection_i == self.selection_ptrs.len() - 1 {
                0
            } else {
                self.selection_i + 1
            };
            self.its_completed += 1;
            phrase
        }
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashSet;

    use super::core::{combinations, random_slugs};

    #[test]
    fn happy_2() {
        assert!(random_slugs(2, Some(1)).unwrap().len() > 0);
    }

    #[test]
    fn unhappy_high() {
        match random_slugs(6, Some(1)) {
            Ok(_v) => assert!(false),
            Err(_e) => assert!(true),
        }
    }

    #[test]
    fn unhappy_low() {
        match random_slugs(0, Some(1)) {
            Ok(_v) => assert!(false),
            Err(_e) => assert!(true),
        }
    }

    #[test]
    fn unhappy_negative() {
        match random_slugs(-1, Some(1)) {
            Ok(_v) => assert!(false),
            Err(_e) => assert!(true),
        }
    }

    #[test]
    fn combinations_happy() {
        let mut combo = 0;
        for i in 1..5 {
            let val = combinations(i).unwrap();
            assert!(val > combo);
            combo += val
        }
    }

    #[test]
    fn combinations_unhappy_high() {
        match combinations(6) {
            Ok(_v) => assert!(false),
            Err(_e) => assert!(true),
        }
    }

    #[test]
    fn combinations_unhappy_low() {
        match combinations(0) {
            Ok(_v) => assert!(false),
            Err(_e) => assert!(true),
        }
    }

    #[test]
    fn combinations_unhappy_negative() {
        match combinations(-1) {
            Ok(_v) => assert!(false),
            Err(_e) => assert!(true),
        }
    }

    #[test]
    fn happy_2_all_unique_half() {
        let combos = combinations(2).unwrap() / 2;
        let slugs = random_slugs(2, Some(combos as i32))
            .expect("unable to create 2 word slugs for all possible combinations");
        assert!(slugs.len() == combos);
        let mut hs = HashSet::new();
        dbg!(&slugs[..10]);
        for slug in slugs {
            hs.insert(slug);
        }
        assert_eq!(hs.len(), combos)
    }

    #[test]
    fn happy_2_all_unique_all() {
        let possible_combos = combinations(2).unwrap();
        let slugs = random_slugs(2, Some(possible_combos as i32))
            .expect("unable to create 2 word slugs for all possible combinations");
        assert!(slugs.len() == possible_combos);
        let mut hs = HashSet::new();
        dbg!(&slugs[..10]);
        for slug in slugs {
            hs.insert(slug);
        }
        assert_eq!(hs.len(), possible_combos)
    }

    #[test]
    fn happy_3_all_unique_1_million() {
        // only generate 10 million to save time because the actual total combinations could be well over half a billion
        let combos = 1_000_000;
        let slugs = random_slugs(3, Some(combos as i32))
            .expect("unable to create 2 word slugs for all possible combinations");
        assert!(slugs.len() == combos);
        let mut hs = HashSet::new();
        dbg!(&slugs[..10]);
        for slug in slugs {
            hs.insert(slug);
        }
        assert_eq!(hs.len(), combos)
    }
    #[test]
    fn happy_4_all_unique_1_million() {
        // only generate 10 million to save time because the actual total combinations could be well over half a billion
        let combos = 1_000_000;
        let slugs = random_slugs(4, Some(combos as i32))
            .expect("unable to create 2 word slugs for all possible combinations");
        assert!(slugs.len() == combos);
        let mut hs = HashSet::new();
        dbg!(&slugs[..10]);
        for slug in slugs {
            hs.insert(slug);
        }
        assert_eq!(hs.len(), combos)
    }

    #[test]
    fn happy_5_all_unique_1_million() {
        // only generate 10 million to save time because the actual total combinations could be well over half a billion
        let combos = 1_000_000;
        let slugs = random_slugs(4, Some(combos as i32))
            .expect("unable to create 2 word slugs for all possible combinations");
        assert!(slugs.len() == combos);
        let mut hs = HashSet::new();
        dbg!(&slugs[..10]);
        for slug in slugs {
            hs.insert(slug);
        }
        assert_eq!(hs.len(), combos)
    }
}
