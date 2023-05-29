use crate::{Base64, Binary, BotToken, Dictionary, Joke, Lyrics, Requester, Text};
use anyhow::Result;

pub struct OthersEndpoint(pub(crate) Requester);

impl OthersEndpoint {
    /// Base64 decoding
    pub async fn decode_base64<T: ToString>(&self, text: T) -> Result<Text> {
        self.0
            .request_with_query("others/base64", &[("decode", text.to_string())])
            .await
    }

    /// Base64 encoding
    pub async fn encode_base64<T: ToString>(&self, text: T) -> Result<Base64> {
        self.0
            .request_with_query("others/base64", &[("encode", text.to_string())])
            .await
    }

    // Binary decoding
    pub async fn decode_binary<T: ToString>(&self, text: T) -> Result<Text> {
        self.0
            .request_with_query("others/binary", &[("decode", text.to_string())])
            .await
    }

    // Binary encoding
    pub async fn encode_binary<T: ToString>(&self, text: T) -> Result<Binary> {
        self.0
            .request_with_query("others/binary", &[("encode", text.to_string())])
            .await
    }

    // Generate a random bottoken
    pub async fn bot_token<T: ToString>(&self, bot_id: Option<T>) -> Result<BotToken> {
        match bot_id {
            Some(bot_id) => {
                self.0
                    .request_with_query("others/bottoken", &[("id", bot_id.to_string())])
                    .await
            }
            None => self.0.request("others/bottoken").await,
        }
    }

    /// Lookup words
    pub async fn dictionary<T: ToString>(&self, word: T) -> Result<Dictionary> {
        self.0
            .request_with_query("others/dictionary", &[("word", word.to_string())])
            .await
    }

    /// Return a random joke
    pub async fn joke(&self) -> Result<Joke> {
        self.0.request("others/joke").await
    }

    /// Look up lyrics for a song
    pub async fn lyrics<T: ToString>(&self, title: T) -> Result<Lyrics> {
        self.0
            .request_with_query("others/lyrics", &[("title", title.to_string())])
            .await
    }
}
