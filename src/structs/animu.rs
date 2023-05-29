use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AnimuQuote {
    pub sentence: String,
    pub character: String,
    pub anime: String,
}
