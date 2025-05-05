mod wordle {
    pub mod answer_history_letter;
    pub mod game_status;
    pub mod word_list;
}
mod api;
use api::AnswerHistoryResponse;
use lazy_static::lazy_static;
use std::sync::Mutex;
use wordle::game_status::GameStatus;

lazy_static! {
    static ref GAME_STATUS: Mutex<GameStatus> = Mutex::new(GameStatus::new("./word_list.txt"));
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
    GAME_STATUS.lock().unwrap().reset()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    *GAME_STATUS.lock().unwrap() = GameStatus::new("./word_list.txt");
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_word, check_word, reset])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
