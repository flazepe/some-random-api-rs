use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Quote {
    pub sentence: String,
    pub character: String,
    pub anime: String,
}
