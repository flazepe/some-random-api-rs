use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Image {
    pub link: String,
}
