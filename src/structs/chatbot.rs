use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ChatBotResponse {
    pub response: String,
}
