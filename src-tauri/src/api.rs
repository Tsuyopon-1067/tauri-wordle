use serde::Serialize;

use crate::wordle::game_status::AnswerHistory;

/// Represents the API response containing wordle answer history
///
/// This structure is used to send the game state to the frontend
/// and indicates whether the history was updated
#[derive(Clone, Serialize)]
pub struct AnswerHistoryResponse {
    /// Contains the sequence of attempted words and their evaluation results
    pub histories: AnswerHistory,

    /// Flag indicating whether this response contains new updates
    /// (true when the history was modified since last request)
    pub is_update: bool,
}
