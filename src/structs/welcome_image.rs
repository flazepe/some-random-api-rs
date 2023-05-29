pub struct WelcomeImage {
    pub template: u8,
    pub background: WelcomeImageBackground,
    pub background_url: String,
    pub username: String,
    pub discriminator: String,
    pub avatar_url: String,
    pub guild_name: String,
    pub member_count: u64,
    pub text_color: WelcomeImageTextColor,
    pub font: u8,
    pub card_type: String,
}

#[derive(Debug)]
pub enum WelcomeImageBackground {
    Stars,
    Stars2,
    RainbowGradient,
    Rainbow,
    Sunset,
    Night,
    BlobDay,
    BlobNight,
    Space,
    Gaming1,
    Gaming3,
    Gaming2,
    Gaming4,
}

#[derive(Debug)]
pub enum WelcomeImageTextColor {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Indigo,
    Purple,
    Pink,
    Black,
    White,
}

impl WelcomeImage {
    /// Create an instnace of [`WelcomeImage`]
    ///
    /// # Examples
    ///
    /// ```
    /// use some_random_api::{WelcomeImage, WelcomeImageBackground, WelcomeImageTextColor};
    ///
    /// WelcomeImage::new(
    ///     "username",
    ///     "discriminator"
    ///     "avatar url",
    ///     "guild name",
    ///     193, // Member count
    /// )
    /// .set_background(WelcomeImageBackground::Stars)
    /// .set_text_color(WelcomeImageTextColor::Red)
    /// .set_leave(true);
    /// ```
    pub fn new<T: ToString, U: ToString, V: ToString, W: ToString>(
        username: T,
        discriminator: U,
        avatar_url: V,
        guild_name: W,
        member_count: u64,
    ) -> Self {
        Self {
            template: 1,
            background: WelcomeImageBackground::Stars,
            background_url: "".into(),
            username: username.to_string(),
            discriminator: discriminator.to_string(),
            avatar_url: avatar_url.to_string(),
            guild_name: guild_name.to_string(),
            member_count,
            text_color: WelcomeImageTextColor::White,
            font: 1,
            card_type: "join".into(),
        }
    }

    /// Sets the welcome image template (1-7)
    pub fn set_template(mut self, template: u8) -> Self {
        self.template = template.max(1).min(7);
        self
    }

    /// Sets the welcome image background
    pub fn set_background(mut self, background: WelcomeImageBackground) -> Self {
        self.background = background;
        self
    }

    /// Sets the welcome image background URL
    pub fn set_background_url<T: ToString>(mut self, background_url: T) -> Self {
        self.background_url = background_url.to_string();
        self
    }

    /// Sets the welcome image font (1-7)
    pub fn set_font(mut self, font: u8) -> Self {
        self.font = font.max(1).min(7);
        self
    }

    /// Sets the welcome image text color
    pub fn set_text_color(mut self, text_color: WelcomeImageTextColor) -> Self {
        self.text_color = text_color;
        self
    }

    /// Sets whether the welcome image is for a member that left
    pub fn set_leave(mut self, leave: bool) -> Self {
        if leave {
            self.card_type = "leave".into();
        }

        self
    }

    /// Builds the welcome image to a query
    pub(crate) fn to_query(&self) -> [(&'static str, String); 8] {
        [
            ("username", self.username.clone()),
            ("avatar", self.avatar_url.clone()),
            ("discriminator", self.discriminator.clone()),
            ("guildName", self.guild_name.clone()),
            ("memberCount", self.member_count.to_string()),
            ("textcolor", format!("{:?}", self.text_color).to_lowercase()),
            ("font", self.font.to_string()),
            ("type", self.card_type.clone()),
        ]
    }
}
