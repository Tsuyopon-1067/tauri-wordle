use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

/// A collection of words loaded from a file with utility methods
///
/// The WordList provides functionality to:
/// - Load words from a text file (one word per line)
/// - Check if a word exists in the list
/// - Get a random word from the list
#[derive(Clone)]
pub struct WordList {
    list: Vec<String>,
}

impl WordList {
    /// Creates a new WordList by loading words from the specified file path
    ///
    /// # Arguments
    /// * `path` - Path to the text file containing words (one per line)
    ///
    /// # Panics
    /// Panics if the file cannot be opened
    pub fn new(path: &PathBuf) -> Self {
        let list = Self::load_word_list(path).unwrap();
        Self { list: list }
    }

    /// Loads words from a text file into a Vec<String>
    ///
    /// # Arguments
    /// * `file_path` - Path to the text file containing words
    ///
    /// # Returns
    /// Result containing the word list or an IO error
    ///
    /// # Notes
    /// - Empty lines are filtered out
    fn load_word_list(file_path: &PathBuf) -> Result<Vec<String>, io::Error> {
        let path = Path::new(file_path);
        let file = File::open(path).expect("Could not open file");
        let reader = io::BufReader::new(file);
        let lines: Vec<String> = reader
            .lines()
            .filter_map(|line| line.ok())
            .filter(|line| !line.is_empty())
            .collect();
        Ok(lines)
    }

    /// Checks if the word exists in the word list
    ///
    /// # Arguments
    /// * `word` - The word to check
    pub fn contains(&self, word: &String) -> bool {
        self.list.contains(word)
    }

    /// Returns a random word from the word list
    ///
    /// # Panics
    /// Panics if the word list is empty
    pub fn get_random_word(&self) -> String {
        let mut rng = thread_rng();
        let random_word = self.list.choose(&mut rng).unwrap();
        random_word.to_string()
    }
}
