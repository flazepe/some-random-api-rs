use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Response {
    pub response: String,
}
