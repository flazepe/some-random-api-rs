pub struct RankCard {
    pub template: u8,
    pub username: String,
    pub discriminator: String,
    pub avatar_url: String,
    pub level: u64,
    pub current_xp: u64,
    pub needed_xp: u64,
    pub background_url: String,
    pub background_color: String,
    pub text_color: String,
    pub current_xp_color: String,
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
    /// .set_background_color(0x000000)
    /// .set_text_color(0xff0000);
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
    pub fn set_background_color(mut self, background_color: u32) -> Self {
        self.background_color = format!("{:06x}", background_color.min(0xffffff));
        self
    }

    /// Sets the rank card text color
    pub fn set_text_color(mut self, text_color: u32) -> Self {
        self.text_color = format!("{:06x}", text_color.min(0xffffff));
        self
    }

    /// Sets the rank card current xp color
    pub fn set_current_xp_color(mut self, current_xp_color: u32) -> Self {
        self.current_xp_color = format!("{:06x}", current_xp_color.min(0xffffff));
        self
    }

    /// Sets the rank card xp bar color
    pub fn set_xp_bar_color(mut self, xp_bar_color: u32) -> Self {
        self.xp_bar_color = format!("{:06x}", xp_bar_color.min(0xffffff));
        self
    }

    /// Creates a query from the rank card
    pub(crate) fn into_query(self) -> [(&'static str, String); 11] {
        [
            ("username", self.username),
            ("discriminator", self.discriminator),
            ("avatar", self.avatar_url),
            ("level", self.level.to_string()),
            ("cxp", self.current_xp.to_string()),
            ("nxp", self.needed_xp.to_string()),
            ("bg", self.background_url),
            ("cbg", self.background_color),
            ("ctext", self.text_color),
            ("ccxp", self.current_xp_color),
            ("cbar", self.xp_bar_color),
        ]
    }
}
