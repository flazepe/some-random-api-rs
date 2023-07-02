use crate::{Hex, Requester, Rgb, Tweet};
use anyhow::{Error, Result};

pub struct CanvasMiscEndpoint(pub(crate) Requester);

impl<'a> CanvasMiscEndpoint {
    /// Add a bisexual flag border to your avatar
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

    /// Crop your avatar to a circle
    pub async fn circle<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image("canvas/misc/circle", &[("avatar", avatar_url.to_string())])
            .await
    }

    /// Generate a square image of a hex color
    pub async fn color_viewer<T: TryInto<Hex>>(&self, hex: T) -> Result<Vec<u8>> {
        self.0
            .request_image(
                "canvas/misc/colorviewer",
                &[(
                    "hex",
                    hex.try_into()
                        .map_err(|_| Error::msg("Failed to parse hex string"))?
                        .hex,
                )],
            )
            .await
    }

    /// Crop your avatar to a heart shape
    pub async fn heart<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image("canvas/misc/heart", &[("avatar", avatar_url.to_string())])
            .await
    }

    /// Convert RGB to hex
    pub async fn hex<T: ToString>(&self, rgb: T) -> Result<Hex> {
        self.0
            .request("canvas/misc/hex", Some(&[("rgb", rgb.to_string())]))
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

    /// Turn your avatar to a JPG
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

    /// Add an LGBT flag border to your avatar
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

    /// Generate a fake Genshin Impact namecard
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

    /// No description?
    pub async fn no_bitches<T: ToString>(&self, no: T) -> Result<Vec<u8>> {
        self.0
            .request_image("canvas/misc/nobitches", &[("no", no.to_string())])
            .await
    }

    /// Add a non-binary flag border to your avatar
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

    /// Pixelate your avatar
    pub async fn pixelate<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image(
                "canvas/misc/pixelate",
                &[("avatar", avatar_url.to_string())],
            )
            .await
    }

    /// Convert hex to RGB
    pub async fn rgb<T: TryInto<Hex, Error = Error>>(&self, hex: T) -> Result<Rgb> {
        self.0
            .request("canvas/misc/rgb", Some(&[("hex", hex.try_into()?.hex)]))
            .await
    }

    /// Generate a simp card
    pub async fn simp<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image(
                "canvas/misc/simpcard",
                &[("avatar", avatar_url.to_string())],
            )
            .await
    }

    /// Generate a spinning avatar
    pub async fn spin<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image("canvas/misc/spin", &[("avatar", avatar_url.to_string())])
            .await
    }

    /// Put an image on a DVD disk (scene from Tonikawa)
    pub async fn tonikawa<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image(
                "canvas/misc/tonikawa",
                &[("avatar", avatar_url.to_string())],
            )
            .await
    }

    /// Add a transgender flag border to your avatar
    pub async fn transgender<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image(
                "canvas/misc/transgender",
                &[("avatar", avatar_url.to_string())],
            )
            .await
    }

    /// Generate a fake tweet
    pub async fn tweet(&self, tweet: Tweet) -> Result<Vec<u8>> {
        self.0.request_image("canvas/misc/tweet", &tweet).await
    }

    /// Generate a fake youtube comment
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
