use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct WordList {
    list: Vec<String>,
}

impl WordList {
    pub fn new(path: &PathBuf) -> Self {
        let list = Self::load_word_list(path).unwrap();
        Self { list: list }
    }

    fn load_word_list(file_path: &PathBuf) -> Result<Vec<String>, io::Error> {
        let path = Path::new(file_path);
        let file = File::open(path).expect("ファイルを開けません");
        let reader = io::BufReader::new(file);
        let lines: Vec<String> = reader
            .lines()
            .filter_map(|line| line.ok())
            .filter(|line| !line.is_empty())
            .collect();
        Ok(lines)
    }

    pub fn contains(&self, word: &String) -> bool {
        self.list.contains(word)
    }

    pub fn get_random_word(&self) -> String {
        let mut rng = thread_rng();
        let random_word = self.list.choose(&mut rng).unwrap();
        random_word.to_string()
    }
}
