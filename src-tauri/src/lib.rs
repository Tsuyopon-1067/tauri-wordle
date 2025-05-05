mod wordle {
    pub mod answer_history_letter;
    pub mod game_status;
    pub mod word_list;
}
mod api;
use api::AnswerHistoryResponse;
use lazy_static::lazy_static;
use std::sync::Mutex;
use tauri::{path::BaseDirectory, Manager};
use wordle::game_status::GameStatus;

lazy_static! {
    static ref GAME_STATUS: Mutex<Option<GameStatus>> = Mutex::new(None);
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn get_word() -> String {
    GAME_STATUS.lock().unwrap().as_mut().unwrap().answer.clone()
}

#[tauri::command]
fn check_word(word: String) -> AnswerHistoryResponse {
    GAME_STATUS
        .lock()
        .unwrap()
        .as_mut()
        .unwrap()
        .push(word)
        .clone()
}

#[tauri::command]
fn reset() -> AnswerHistoryResponse {
    GAME_STATUS.lock().unwrap().as_mut().unwrap().reset()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let path = app
                .path()
                .resolve("resources/word_list.txt", BaseDirectory::Resource)?;
            *GAME_STATUS.lock().unwrap() = Some(GameStatus::new(&path));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_word, check_word, reset])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
