use super::answer_history_letter::{AnswerHistoryLetter, LetterStatus};
use super::word_list::WordList;
use crate::api::AnswerHistoryResponse;

pub type AnswerHistory = Vec<Vec<AnswerHistoryLetter>>;
/// Manages the state of a Wordle game
///
/// Manages the game's answer, guess history, and completion status
/// # Fields
/// - `answer`: The correct word to guess
/// - `histories`: Player's guess history with letter statuses
/// - `is_clear`: Whether the game has been won
/// - `word_list`: The dictionary of valid words
#[derive(Clone)]
pub struct GameStatus {
    pub answer: String,
    pub histories: AnswerHistory,
    pub is_clear: bool,
    pub word_list: WordList,
}

use std::fmt;
use std::path::PathBuf;

impl fmt::Display for GameStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Answer: {}", self.answer)?;
        writeln!(f, "Histories:")?;
        writeln!(f, "len: {}", self.histories.len())?;
        for row in &self.histories {
            for letter in row {
                match letter.status {
                    LetterStatus::Correct => write!(f, "({}) ", letter.letter)?,
                    LetterStatus::Present => write!(f, "[{}] ", letter.letter)?,
                    LetterStatus::Absent => write!(f, "{{{}}}", letter.letter)?,
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "is_clear: {}", self.is_clear)
    }
}

impl GameStatus {
    /// Creates a new GameStatus instance
    ///
    /// # Arguments
    /// * `path` - Path to the word list file
    ///
    /// # Returns
    /// New GameStatus with random answer word
    ///
    /// # Examples
    /// ```
    /// let path = PathBuf::from("./resources/word_list.txt");
    /// let game = GameStatus::new(&path);
    /// ```
    pub fn new(path: &PathBuf) -> Self {
        let histories = Vec::new();
        let word_list = WordList::new(path);
        let answer = word_list.get_random_word();

        Self {
            answer,
            histories,
            is_clear: false,
            word_list: WordList::new(path),
        }
    }

    /// Processes a player's guess and updates game state
    ///
    /// # Arguments
    /// * `word` - The guessed word
    ///
    /// # Returns
    /// AnswerHistoryResponse containing updated history and status
    ///
    /// # Notes
    /// Only valid words of correct length are processed
    /// Game stops accepting guesses after being won
    pub fn push(&mut self, word: String) -> AnswerHistoryResponse {
        let word = word.to_uppercase();
        if word.len() != self.answer.len()
            || !self.word_list.contains(&word)
            || self.is_clear
            || self.histories.len() >= 6
        {
            return AnswerHistoryResponse {
                histories: self.histories.clone(),
                is_update: false,
            };
        }
        let answer_chars: Vec<char> = self.answer.chars().collect();
        let row: Vec<AnswerHistoryLetter> = word
            .chars()
            .enumerate()
            .map(|(i, c)| {
                let status = if c == answer_chars[i] {
                    LetterStatus::Correct
                } else if answer_chars.contains(&c) {
                    LetterStatus::Present
                } else {
                    LetterStatus::Absent
                };
                AnswerHistoryLetter::new(c, status)
            })
            .collect();
        self.histories.push(row);
        let answer_chars: Vec<char> = self.answer.chars().collect();
        if word.chars().enumerate().all(|(i, c)| c == answer_chars[i]) {
            self.is_clear = true;
        }

        AnswerHistoryResponse {
            histories: self.histories.clone(),
            is_update: true,
        }
    }

    /// Resets the game with a new random word
    ///
    /// # Returns
    /// AnswerHistoryResponse with cleared history
    pub fn reset(&mut self) -> AnswerHistoryResponse {
        self.answer = self.word_list.get_random_word();
        self.histories.clear();
        self.is_clear = false;
        AnswerHistoryResponse {
            histories: self.histories.clone(),
            is_update: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path;

    use super::*;

    fn cureate_game_status() -> GameStatus {
        let path = path::PathBuf::from("./resources/test_word_list.txt");
        let mut status = GameStatus::new(&path);
        status.answer = "APPLE".to_string();
        status
    }

    #[test]
    fn test_push_word_correct() {
        let mut status = cureate_game_status();
        let response = status.push("apple".to_string());
        assert!(status.is_clear);
        assert!(response.histories[0]
            .iter()
            .all(|l| matches!(l.status, LetterStatus::Correct)));
        assert!(response.is_update);
    }

    #[test]
    fn test_check_word_present_letters() {
        let mut status = cureate_game_status();
        let response = status.push("grape".to_string());
        assert!(!status.is_clear);
        assert_eq!(status.histories.len(), 1);
        let statuses: Vec<LetterStatus> = response.histories[0]
            .iter()
            .map(|l| l.status.clone())
            .collect();
        assert_eq!(
            statuses,
            vec![
                LetterStatus::Absent,  // a, g
                LetterStatus::Absent,  // p, r
                LetterStatus::Present, // p, a
                LetterStatus::Present, // l, p
                LetterStatus::Correct  // e, e
            ]
        );
        assert!(response.is_update);
    }

    #[test]
    fn test_check_word_absent_letters() {
        let mut status = cureate_game_status();
        let response = status.push("onion".to_string());
        assert!(!status.is_clear);
        assert_eq!(response.histories.len(), 1);
        // apple
        // onion
        assert!(response.histories[0]
            .iter()
            .all(|l| matches!(l.status, LetterStatus::Absent)));
        assert!(response.is_update);
    }

    #[test]
    fn test_check_word_invalid_word() {
        let mut status = cureate_game_status();
        let initial_history_count = status.histories.len();
        let response = status.push("xyzzy".to_string());
        assert_eq!(response.histories.len(), initial_history_count);
        assert!(!response.is_update);
    }

    #[test]
    fn test_check_word_wrong_length() {
        let mut status = cureate_game_status();
        let initial_history_count = status.histories.len();
        let response = status.push("banana".to_string());
        assert_eq!(response.histories.len(), initial_history_count);
        assert!(!response.is_update);
    }

    #[test]
    fn test_check_word_not_exist() {
        let mut status = cureate_game_status();
        let initial_history_count = status.histories.len();
        let response = status.push("abcde".to_string());
        assert_eq!(response.histories.len(), initial_history_count);
        assert!(!response.is_update);
    }

    #[test]
    fn test_check_after_clear_cannot_push() {
        let mut status = cureate_game_status();
        let response = status.push("apple".to_string());
        assert!(status.is_clear);
        assert_eq!(response.histories.len(), 1);
        assert!(response.is_update);

        let response = status.push("peach".to_string());
        assert_eq!(response.histories.len(), 1);
        assert!(!response.is_update);
    }

    #[test]
    fn test_clear_reset() {
        let mut status = cureate_game_status();
        let response = status.push("apple".to_string());
        assert!(status.is_clear);
        assert!(response.is_update);
        status.reset();
        assert!(!status.is_clear);
        assert!(response.is_update);
    }
}
