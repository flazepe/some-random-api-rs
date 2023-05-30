use crate::{ChatBotResponse, Requester};
use anyhow::Result;

/// An endpoint for interactiong with the ChatBot
///
/// # Examples
///
/// ```
/// use some_random_api::Client;
/// use std::fs::write;
///
/// Client::new(Some("xxxxxxxxxx")).chatbot.chatbot("Hello there").await?;
/// ```
pub struct ChatBotEndpoint(pub(crate) Requester);

impl ChatBotEndpoint {
    /// Talk to a virtual chatbot
    pub async fn chatbot<T: ToString>(&self, message: T) -> Result<ChatBotResponse> {
        self.0
            .request("chatbot", Some(&[("message", message.to_string())]))
            .await
    }
}
