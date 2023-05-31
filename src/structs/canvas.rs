use anyhow::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Hex {
    pub hex: String,
}

impl TryFrom<u32> for Hex {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(Self {
            hex: format!("{:06x}", value.min(0xffffff)),
        })
    }
}

impl TryFrom<&str> for Hex {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            hex: format!(
                "{:06x}",
                u32::from_str_radix(value.strip_prefix("#").unwrap_or(&value), 16)?.min(0xffffff),
            ),
        })
    }
}

impl TryFrom<String> for Hex {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self {
            hex: format!(
                "{:06x}",
                u32::from_str_radix(value.strip_prefix("#").unwrap_or(&value), 16)?.min(0xffffff),
            ),
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
