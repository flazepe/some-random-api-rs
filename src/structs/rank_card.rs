use crate::Hex;
use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RankCard {
    #[serde(skip_serializing)]
    pub template: u8,

    pub username: String,
    pub discriminator: String,

    #[serde(rename = "avatar")]
    pub avatar_url: String,

    pub level: u64,

    #[serde(rename = "cxp")]
    pub current_xp: u64,

    #[serde(rename = "nxp")]
    pub needed_xp: u64,

    #[serde(rename = "bg")]
    pub background_url: String,

    #[serde(rename = "cbg")]
    pub background_color: String,

    #[serde(rename = "ctext")]
    pub text_color: String,

    #[serde(rename = "ccxp")]
    pub current_xp_color: String,

    #[serde(rename = "cbar")]
    pub xp_bar_color: String,
}

impl RankCard {
    /// Create an instnace of [`RankCard`]
    ///
    /// # Examples
    ///
    /// ```
    /// use some_random_api::RankCard;
    ///
    /// RankCard::new(
    ///     "username",
    ///     "discriminator"
    ///     "avatar url",
    ///     50, // Level
    ///     (50, 100), // (current xp, needed xp)
    /// )
    /// .set_background_color(0x000000)?
    /// .set_text_color(0xff0000)?;
    /// ```
    pub fn new<T: ToString, U: ToString, V: ToString>(
        username: T,
        discriminator: U,
        avatar_url: V,
        level: u64,
        (current_xp, needed_xp): (u64, u64),
    ) -> Self {
        Self {
            template: 1,
            username: username.to_string(),
            discriminator: discriminator.to_string(),
            avatar_url: avatar_url.to_string(),
            level,
            current_xp,
            needed_xp,
            background_url: "".into(),
            background_color: "".into(),
            text_color: "".into(),
            current_xp_color: "".into(),
            xp_bar_color: "".into(),
        }
    }

    /// Sets the rank card template (1-8)
    pub fn set_template(mut self, template: u8) -> Self {
        self.template = template.max(1).min(8);
        self
    }

    /// Sets the rank card background URL
    pub fn set_background_url<T: ToString>(mut self, background_url: T) -> Self {
        self.background_url = background_url.to_string();
        self
    }

    /// Sets the rank card background color
    pub fn set_background_color<T: TryInto<Hex>>(mut self, hex: T) -> Result<Self, T::Error> {
        self.background_color = hex.try_into()?.hex;
        Ok(self)
    }

    /// Sets the rank card text color
    pub fn set_text_color<T: TryInto<Hex>>(mut self, hex: T) -> Result<Self, T::Error> {
        self.text_color = hex.try_into()?.hex;
        Ok(self)
    }

    /// Sets the rank card current xp color
    pub fn set_current_xp_color<T: TryInto<Hex>>(mut self, hex: T) -> Result<Self, T::Error> {
        self.current_xp_color = hex.try_into()?.hex;
        Ok(self)
    }

    /// Sets the rank card xp bar color
    pub fn set_xp_bar_color<T: TryInto<Hex>>(mut self, hex: T) -> Result<Self, T::Error> {
        self.xp_bar_color = hex.try_into()?.hex;
        Ok(self)
    }
}
