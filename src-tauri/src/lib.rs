use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Serialize;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::Mutex;

lazy_static! {
    static ref GAME_STATUS: Mutex<GameStatus> = Mutex::new(GameStatus::new(String::new()));
    static ref WORD_LIST: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

type AnswerHistory = Vec<Vec<AnswerHistoryLetter>>;
#[derive(Clone, Serialize)]
pub struct GameStatus {
    pub answer: String,
    pub histories: AnswerHistory,
    pub is_clear: bool,
}

impl GameStatus {
    pub fn new(answer: String) -> Self {
        let answer = answer.to_uppercase();
        let histories = Vec::new();
        Self {
            answer,
            histories,
            is_clear: false,
        }
    }

    pub fn push(&mut self, word: String) -> AnswerHistoryResponse {
        let word = word.to_uppercase();
        if word.len() != self.answer.len()
            || !WORD_LIST.lock().unwrap().contains(&word)
            || self.is_clear
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

    pub fn reset(&mut self, new_answer: String) -> AnswerHistoryResponse {
        self.histories.clear();
        self.is_clear = false;
        self.answer = new_answer;
        AnswerHistoryResponse {
            histories: self.histories.clone(),
            is_update: true,
        }
    }
}

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

#[derive(Clone, Serialize)]
pub struct AnswerHistoryResponse {
    pub histories: AnswerHistory,
    pub is_update: bool,
}

fn load_word_list(file_path: &str) -> Result<(), io::Error> {
    let path = Path::new(file_path);
    let file = File::open(path).expect("ファイルを開けません");
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.is_empty())
        .collect();
    *WORD_LIST.lock().unwrap() = lines;
    Ok(())
}

fn get_random_word() -> io::Result<String> {
    let lines: Vec<String> = WORD_LIST.lock().unwrap().clone();
    if lines.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "ファイルは空です",
        ));
    }

    let mut rng = thread_rng();
    match lines.choose(&mut rng) {
        Some(line) => Ok(line.clone()),
        None => Err(io::Error::new(
            io::ErrorKind::Other,
            "ランダムな行を選択できませんでした",
        )),
    }
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn get_word() -> String {
    GAME_STATUS.lock().unwrap().answer.clone()
}

#[tauri::command]
fn check_word(word: String) -> AnswerHistoryResponse {
    GAME_STATUS.lock().unwrap().push(word).clone()
}

#[tauri::command]
fn reset() -> AnswerHistoryResponse {
    let random_word = match get_random_word() {
        Ok(word) => word,
        Err(err) => panic!("error occurred while selecting word : {}", err),
    };
    GAME_STATUS.lock().unwrap().reset(random_word).clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() {
        load_word_list("./test_word_list.txt").unwrap();
        *GAME_STATUS.lock().unwrap() = GameStatus::new("apple".to_string());
    }

    fn get_game_status() -> GameStatus {
        GAME_STATUS.lock().unwrap().clone()
    }

    #[test]
    fn test_check_word_correct() {
        setup();
        let response = check_word("apple".to_string());
        let game_status = get_game_status();
        assert!(game_status.is_clear);
        assert!(response.histories[0]
            .iter()
            .all(|l| matches!(l.status, LetterStatus::Correct)));
        assert!(response.is_update);
    }

    #[test]
    fn test_check_word_present_letters() {
        setup();
        let response = check_word("grape".to_string());
        let game_status = get_game_status();
        assert!(!game_status.is_clear);
        assert_eq!(game_status.histories.len(), 1);
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
        setup();
        let response = check_word("onion".to_string());
        let game_status = get_game_status();
        assert!(!game_status.is_clear);
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
        setup();
        let game_status = get_game_status();
        let initial_history_count = game_status.histories.len();
        let response = check_word("xyzzy".to_string());
        assert_eq!(response.histories.len(), initial_history_count);
        assert!(!response.is_update);
    }

    #[test]
    fn test_check_word_wrong_length() {
        setup();
        let game_status = get_game_status();
        let initial_history_count = game_status.histories.len();
        let response = check_word("banana".to_string());
        assert_eq!(response.histories.len(), initial_history_count);
        assert!(!response.is_update);
    }

    #[test]
    fn test_check_word_not_exist() {
        setup();
        let game_status = get_game_status();
        let initial_history_count = game_status.histories.len();
        let response = check_word("abcde".to_string());
        assert_eq!(response.histories.len(), initial_history_count);
        assert!(!response.is_update);
    }

    #[test]
    fn test_check_after_clear_cannot_push() {
        setup();
        let response = check_word("apple".to_string());
        let game_status = get_game_status();
        assert!(game_status.is_clear);
        assert_eq!(response.histories.len(), 1);
        assert!(response.is_update);

        let response = check_word("peach".to_string());
        assert_eq!(response.histories.len(), 1);
        assert!(!response.is_update);
    }

    #[test]
    fn test_clear_reset() {
        setup();
        let response = check_word("apple".to_string());
        let game_status = get_game_status();
        assert!(game_status.is_clear);
        assert!(response.is_update);
        let response = GAME_STATUS.lock().unwrap().reset("grape".to_string());
        let game_status = get_game_status();
        assert!(!game_status.is_clear);
        assert!(response.is_update);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    match load_word_list("./word_list.txt") {
        Ok(_) => {}
        Err(err) => panic!("error occurred while loading the file : {}", err),
    };
    let random_word = match get_random_word() {
        Ok(word) => word,
        Err(err) => panic!("error occurred while selecting word : {}", err),
    };
    *GAME_STATUS.lock().unwrap() = GameStatus::new(random_word);
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_word, check_word, reset])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
