use serde::Serialize;
use serde_repr::Serialize_repr;

#[derive(Debug, Serialize)]
pub struct WelcomeImage {
    #[serde(skip_serializing)]
    pub template: WelcomeImageTemplate,

    #[serde(skip_serializing)]
    pub background: WelcomeImageBackground,

    #[serde(rename = "bg")]
    pub background_url: String,

    #[serde(rename = "type")]
    pub card_type: String,

    pub username: String,
    pub discriminator: String,

    #[serde(rename = "avatar")]
    pub avatar_url: String,

    #[serde(rename = "guildName")]
    pub guild_name: String,

    #[serde(rename = "memberCount")]
    pub member_count: u64,

    #[serde(rename = "textcolor")]
    pub text_color: WelcomeImageTextColor,

    pub font: u8,
}

#[derive(Debug, Serialize_repr)]
#[repr(u8)]
pub enum WelcomeImageTemplate {
    A = 1,
    B,
    C,
    D,
    E,
    F,
    G,
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
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
    ///     "avatar url",
    ///     "guild name",
    ///     100, // Member count
    /// )
    /// .set_background(WelcomeImageBackground::Stars)
    /// .set_text_color(WelcomeImageTextColor::Red)
    /// .set_leave(true);
    /// ```
    pub fn new<T: ToString, U: ToString, V: ToString>(
        username: T,
        avatar_url: U,
        guild_name: V,
        member_count: u64,
    ) -> Self {
        Self {
            template: WelcomeImageTemplate::A,
            background: WelcomeImageBackground::Stars,
            background_url: "".into(),
            card_type: "join".into(),
            username: username.to_string(),
            discriminator: "0".into(),
            avatar_url: avatar_url.to_string(),
            guild_name: guild_name.to_string(),
            member_count,
            text_color: WelcomeImageTextColor::White,
            font: 1,
        }
    }

    /// Sets the welcome image template
    pub fn set_template(mut self, template: WelcomeImageTemplate) -> Self {
        self.template = template;
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

    /// Sets the welcome image user discriminator
    pub fn set_discriminator<T: ToString>(mut self, discriminator: T) -> Self {
        self.discriminator = discriminator.to_string();
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
}
