use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Hex {
    pub hex: String,
}

#[derive(Debug, Deserialize)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
