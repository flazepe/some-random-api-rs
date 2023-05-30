pub struct Tweet {
    pub username: String,
    pub display_name: String,
    pub avatar_url: String,
    pub comment: String,
    pub replies: String,
    pub retweets: String,
    pub likes: String,
    pub theme: String,
}

impl Tweet {
    /// Create an instance of [`Tweet`]
    ///
    /// # Examples
    ///
    /// ```
    /// use some_random_api::Tweet;
    ///
    /// Tweet::new(
    ///     "username",
    ///     "avatar url",
    ///     "comment"
    /// )
    /// .set_display_name("display name")
    /// .set_dark_theme(true)
    /// .set_replies("100K");
    /// ```
    pub fn new<T: ToString, U: ToString, V: ToString>(
        username: T,
        avatar_url: U,
        comment: V,
    ) -> Self {
        Self {
            username: username.to_string(),
            display_name: username.to_string(),
            avatar_url: avatar_url.to_string(),
            comment: comment.to_string(),
            replies: "".into(),
            retweets: "".into(),
            likes: "".into(),
            theme: "".into(),
        }
    }

    /// Sets the display name of the tweet author
    pub fn set_display_name<T: ToString>(mut self, display_name: T) -> Self {
        self.display_name = display_name.to_string();
        self
    }

    /// Sets the tweet reply amount or text
    pub fn set_replies<T: ToString>(mut self, replies: T) -> Self {
        self.replies = replies.to_string();
        self
    }

    /// Sets the tweet retweet amount or text
    pub fn set_retweets<T: ToString>(mut self, retweets: T) -> Self {
        self.retweets = retweets.to_string();
        self
    }

    /// Sets the tweet like amount or text
    pub fn set_likes<T: ToString>(mut self, likes: T) -> Self {
        self.likes = likes.to_string();
        self
    }

    /// Sets whether to use dark theme instead
    pub fn set_dark_theme(mut self, dark_theme: bool) -> Self {
        if dark_theme {
            self.theme = "dark".into();
        }

        self
    }

    /// Builds the tweet to a query
    pub(crate) fn into_query(self) -> [(&'static str, String); 8] {
        [
            ("username", self.username),
            ("displayname", self.display_name),
            ("avatar", self.avatar_url),
            ("comment", self.comment),
            ("replies", self.replies),
            ("retweets", self.retweets),
            ("likes", self.likes),
            ("theme", self.theme),
        ]
    }
}
