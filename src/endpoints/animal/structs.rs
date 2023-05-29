use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Animal {
    pub image: String,
    pub fact: String,
}
