use crate::{Base64, Binary, BotToken, Dictionary, Joke, Lyrics, Requester, Text};
use anyhow::Result;

/// An endpoint that sends other random stuff
///
/// # Examples
///
/// ```
/// use some_random_api::Client;
///
/// Client::new(None::<String>).others.joke().await?;
/// ```
pub struct OthersEndpoint(pub(crate) Requester);

impl OthersEndpoint {
    /// Decode Base64
    pub async fn decode_base64<T: ToString>(&self, text: T) -> Result<Text> {
        self.0
            .request("others/base64", Some(&[("decode", text.to_string())]))
            .await
    }

    /// Encode Base64
    pub async fn encode_base64<T: ToString>(&self, text: T) -> Result<Base64> {
        self.0
            .request("others/base64", Some(&[("encode", text.to_string())]))
            .await
    }

    // Decode binary
    pub async fn decode_binary<T: ToString>(&self, text: T) -> Result<Text> {
        self.0
            .request("others/binary", Some(&[("decode", text.to_string())]))
            .await
    }

    // Encode binary
    pub async fn encode_binary<T: ToString>(&self, text: T) -> Result<Binary> {
        self.0
            .request("others/binary", Some(&[("encode", text.to_string())]))
            .await
    }

    // Generate a random Discord bot token
    pub async fn bot_token<T: ToString>(&self, bot_id: Option<T>) -> Result<BotToken> {
        match bot_id {
            Some(bot_id) => {
                self.0
                    .request("others/bottoken", Some(&[("id", bot_id.to_string())]))
                    .await
            }
            None => self.0.request("others/bottoken", None::<&()>).await,
        }
    }

    /// Look up words
    pub async fn dictionary<T: ToString>(&self, word: T) -> Result<Dictionary> {
        self.0
            .request("others/dictionary", Some(&[("word", word.to_string())]))
            .await
    }

    /// Generate a random joke
    pub async fn joke(&self) -> Result<Joke> {
        self.0.request("others/joke", None::<&()>).await
    }

    /// Look up lyrics for a song
    pub async fn lyrics<T: ToString>(&self, title: T) -> Result<Lyrics> {
        self.0
            .request("others/lyrics", Some(&[("title", title.to_string())]))
            .await
    }
}
