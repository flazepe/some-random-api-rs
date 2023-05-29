use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Fact {
    pub fact: String,
}
