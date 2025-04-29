use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::Mutex;

lazy_static! {
    static ref GAME_STATUS: Mutex<GameStatus> = Mutex::new(GameStatus::new(String::new()));
}

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
}

#[derive(Debug, Clone)]
pub enum LetterStatus {
    Correct,
    Present,
    Absent,
}

#[derive(Debug, Clone)]
pub struct AnswerHistoryLetter {
    pub letter: char,
    pub status: LetterStatus,
}

impl AnswerHistoryLetter {
    pub fn new(letter: char, status: LetterStatus) -> Self {
        Self { letter, status }
    }
}

fn get_random_word(file_path: &str) -> io::Result<String> {
    let path = Path::new(file_path);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let lines: Vec<String> = reader.lines().collect::<io::Result<_>>()?;

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let random_word = match get_random_word("./word_list.txt") {
        Ok(word) => word,
        Err(err) => panic!("error occurred while loading the file : {}", err),
    };
    *GAME_STATUS.lock().unwrap() = GameStatus::new(random_word);
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_word])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
