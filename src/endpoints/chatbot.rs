use crate::{ChatBotResponse, Requester};
use anyhow::Result;

pub struct ChatBotEndpoint(pub(crate) Requester);

impl ChatBotEndpoint {
    /// Talk to a virtual chatbot
    pub async fn chatbot<T: ToString>(&self, message: T) -> Result<ChatBotResponse> {
        self.0
            .request_with_query("chatbot", &[("message", message.to_string())])
            .await
    }
}
