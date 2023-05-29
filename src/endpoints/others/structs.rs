use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Base64 {
    pub base64: String,
}

#[derive(Debug, Deserialize)]
pub struct Binary {
    pub binary: String,
}

#[derive(Debug, Deserialize)]
pub struct DecodedText {
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct BotToken {
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct Dictionary {
    pub word: String,
    pub definition: String,
}

#[derive(Debug, Deserialize)]
pub struct Joke {
    pub joke: String,
}

#[derive(Debug, Deserialize)]
pub struct Lyrics {
    pub title: String,
    pub author: String,
    pub lyrics: String,
}
