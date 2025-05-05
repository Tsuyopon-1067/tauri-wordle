use serde::Serialize;

use crate::wordle::game_status::AnswerHistory;

#[derive(Clone, Serialize)]
pub struct AnswerHistoryResponse {
    pub histories: AnswerHistory,
    pub is_update: bool,
}
