use crate::{
    endpoints::canvas::structs::{Hex, RGB},
    requester::Requester,
    Tweet,
};
use anyhow::Result;

pub struct CanvasMiscEndpoint(pub(crate) Requester);

impl<'a> CanvasMiscEndpoint {
    /// Add a bisex flag border to your avatar
    pub async fn bisexual<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image(
                "canvas/misc/bisexual",
                &[("avatar", avatar_url.to_string())],
            )
            .await
    }

    /// Blur your avatar
    pub async fn blur<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image("canvas/misc/blur", &[("avatar", avatar_url.to_string())])
            .await
    }

    /// Crop an image to a circle
    pub async fn circle<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image("canvas/misc/circle", &[("avatar", avatar_url.to_string())])
            .await
    }

    /// View a color
    pub async fn color(&self, hex: u32) -> Result<Vec<u8>> {
        self.0
            .request_image(
                "canvas/misc/colorviewer",
                &[("hex", format!("#{:06x}", hex.min(0xffffff)))],
            )
            .await
    }

    /// Crop an image to a heart shape
    pub async fn heart<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image("canvas/misc/heart", &[("avatar", avatar_url.to_string())])
            .await
    }

    /// Convert rgb to hex
    pub async fn hex<T: ToString>(&self, rgb: T) -> Result<Hex> {
        self.0
            .request_with_query("canvas/misc/hex", &[("rgb", rgb.to_string())])
            .await
    }

    /// Horny card
    pub async fn horny<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image("canvas/misc/horny", &[("avatar", avatar_url.to_string())])
            .await
    }

    /// You are so stupid
    pub async fn stupid<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image(
                "canvas/misc/its-so-stupid",
                &[("avatar", avatar_url.to_string())],
            )
            .await
    }

    /// Jpg/Blur your avatar
    pub async fn jpg<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image("canvas/misc/jpg", &[("avatar", avatar_url.to_string())])
            .await
    }

    /// Add a lesbian flag border to your avatar
    pub async fn lesbian<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image("canvas/misc/lesbian", &[("avatar", avatar_url.to_string())])
            .await
    }

    /// Add a lgbt flag border to your avatar
    pub async fn lgbt<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image("canvas/misc/lgbt", &[("avatar", avatar_url.to_string())])
            .await
    }

    /// I lied to you as naturally as idk tbh
    pub async fn lied<T: ToString, U: ToString>(
        &self,
        username: T,
        avatar_url: U,
    ) -> Result<Vec<u8>> {
        self.0
            .request_image(
                "canvas/misc/lied",
                &[
                    ("username", username.to_string()),
                    ("avatar", avatar_url.to_string()),
                ],
            )
            .await
    }

    /// Loli police
    pub async fn lolice<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image("canvas/misc/lolice", &[("avatar", avatar_url.to_string())])
            .await
    }

    /// Genshin namecard
    pub async fn namecard<T: ToString, U: ToString, V: ToString, W: ToString>(
        &self,
        username: T,
        avatar_url: U,
        birthday: V,
        description: Option<W>,
    ) -> Result<Vec<u8>> {
        self.0
            .request_image(
                "canvas/misc/namecard",
                &[
                    ("username", username.to_string()),
                    ("avatar", avatar_url.to_string()),
                    ("birthday", birthday.to_string()),
                    (
                        "description",
                        description.map_or("".into(), |description| description.to_string()),
                    ),
                ],
            )
            .await
    }

    /// No Description?
    pub async fn no_bitches<T: ToString>(&self, no: T) -> Result<Vec<u8>> {
        self.0
            .request_image("canvas/misc/nobitches", &[("no", no.to_string())])
            .await
    }

    /// Add a nonbinary flag border to your avatar
    pub async fn non_binary<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image(
                "canvas/misc/nonbinary",
                &[("avatar", avatar_url.to_string())],
            )
            .await
    }

    /// Share Master Oogway's wisdom
    pub async fn oogway<T: ToString>(&self, quote: T) -> Result<Vec<u8>> {
        self.0
            .request_image("canvas/misc/oogway", &[("quote", quote.to_string())])
            .await
    }

    /// Share Master Oogway's wisdom (different image)
    pub async fn oogway_2<T: ToString>(&self, quote: T) -> Result<Vec<u8>> {
        self.0
            .request_image("canvas/misc/oogway2", &[("quote", quote.to_string())])
            .await
    }

    /// Add a pansexual flag border to your avatar
    pub async fn pansexual<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image(
                "canvas/misc/pansexual",
                &[("avatar", avatar_url.to_string())],
            )
            .await
    }

    /// Pixelate your avataar
    pub async fn pixelate<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image(
                "canvas/misc/pixelate",
                &[("avatar", avatar_url.to_string())],
            )
            .await
    }

    /// Convert hex to rgb
    pub async fn rgb(&self, hex: u32) -> Result<RGB> {
        self.0
            .request_with_query(
                "canvas/misc/rgb",
                &[("hex", format!("{:06x}", hex.min(0xffffff)))],
            )
            .await
    }

    /// Simp card
    pub async fn simp<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image(
                "canvas/misc/simpcard",
                &[("avatar", avatar_url.to_string())],
            )
            .await
    }

    /// You spin me round right round
    pub async fn spin<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image("canvas/misc/spin", &[("avatar", avatar_url.to_string())])
            .await
    }

    /// Put an image on a dvd disk (scene from tonikawa)
    pub async fn tonikawa<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image(
                "canvas/misc/tonikawa",
                &[("avatar", avatar_url.to_string())],
            )
            .await
    }

    /// Add a trans flag border to your avatar
    pub async fn transgender<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image(
                "canvas/misc/transgender",
                &[("avatar", avatar_url.to_string())],
            )
            .await
    }

    /// Create a fake tweet
    pub async fn tweet(&self, tweet: Tweet) -> Result<Vec<u8>> {
        self.0
            .request_image("canvas/misc/tweet", &tweet.to_query())
            .await
    }

    /// Create a fake youtube comment
    pub async fn youtube_comment<T: ToString, U: ToString, V: ToString>(
        &self,
        username: T,
        avatar_url: U,
        comment: V,
    ) -> Result<Vec<u8>> {
        self.0
            .request_image(
                "canvas/misc/youtube-comment",
                &[
                    ("username", username.to_string()),
                    ("avatar", avatar_url.to_string()),
                    ("comment", comment.to_string()),
                ],
            )
            .await
    }
}