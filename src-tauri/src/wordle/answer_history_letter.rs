use serde::Serialize;

/// Represents the evaluation status of a letter in Wordle
#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum LetterStatus {
    /// Letter is in the correct position
    Correct,
    /// Letter exists in the word but in wrong position
    Present,
    /// Letter does not exist in the word
    Absent,
}

/// Represents a single letter in the guess history with its evaluation status
#[derive(Debug, Clone, Serialize)]
pub struct AnswerHistoryLetter {
    /// The guessed character
    pub letter: char,
    /// The evaluation status of the letter
    pub status: LetterStatus,
}

impl AnswerHistoryLetter {
    /// Creates a new AnswerHistoryLetter
    ///
    /// # Arguments
    /// * `letter` - The character guessed
    /// * `status` - Evaluation status of the letter
    pub fn new(letter: char, status: LetterStatus) -> Self {
        Self { letter, status }
    }
}
