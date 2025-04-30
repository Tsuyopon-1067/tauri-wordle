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

#[derive(Clone, Serialize)]
pub struct GameStatus {
    pub answer: String,
    pub histories: Vec<Vec<AnswerHistoryLetter>>,
    pub is_clear: bool,
}

impl GameStatus {
    pub fn new(answer: String) -> Self {
        let histories = Vec::new();
        Self {
            answer,
            histories,
            is_clear: false,
        }
    }

    pub fn push(&mut self, word: String) -> &Self {
        if word.len() != self.answer.len() || !WORD_LIST.lock().unwrap().contains(&word) {
            return self;
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
        self
    }
}

#[derive(Debug, Clone, Serialize)]
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
fn check_word(word: String) -> GameStatus {
    GAME_STATUS.lock().unwrap().push(word).clone()
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
        .invoke_handler(tauri::generate_handler![greet, get_word, check_word])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
