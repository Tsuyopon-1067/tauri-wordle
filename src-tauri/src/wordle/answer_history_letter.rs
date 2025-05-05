use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum LetterStatus {
    Correct,
    Present,
    Absent,
}

#[derive(Debug, Clone, Serialize)]
pub struct AnswerHistoryLetter {
    pub letter: char,
    pub status: LetterStatus,
}

impl AnswerHistoryLetter {
    pub fn new(letter: char, status: LetterStatus) -> Self {
        Self { letter, status }
    }
}
