use crate::{Hex, Requester};
use anyhow::{Error, Result};

pub struct CanvasFilterEndpoint(pub(crate) Requester);

impl<'a> CanvasFilterEndpoint {
    /// Blueify(?) your avatar
    pub async fn blue<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image("canvas/filter/blue", &[("avatar", avatar_url.to_string())])
            .await
    }

    /// Blurplify your avatar
    pub async fn blurple<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image(
                "canvas/filter/blurple",
                &[("avatar", avatar_url.to_string())],
            )
            .await
    }

    /// Blurplify your avatar (using new Discord blurple)
    pub async fn blurple_2<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image(
                "canvas/filter/blurple2",
                &[("avatar", avatar_url.to_string())],
            )
            .await
    }

    /// Brighten your avatar
    pub async fn brightness<T: ToString>(&self, avatar_url: T, brightness: u8) -> Result<Vec<u8>> {
        self.0
            .request_image(
                "canvas/filter/brightness",
                &[
                    ("avatar", avatar_url.to_string()),
                    ("brightness", brightness.min(100).to_string()),
                ],
            )
            .await
    }

    /// Tint your avatar to a certain color
    pub async fn color<T: ToString, U: TryInto<Hex, Error = Error>>(
        &self,
        avatar_url: T,
        hex: U,
    ) -> Result<Vec<u8>> {
        self.0
            .request_image(
                "canvas/filter/color",
                &[
                    ("avatar", avatar_url.to_string()),
                    ("color", hex.try_into()?.hex),
                ],
            )
            .await
    }

    /// Greenify(?) your avatar
    pub async fn green<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image("canvas/filter/green", &[("avatar", avatar_url.to_string())])
            .await
    }

    /// Greyscale your avatar
    pub async fn greyscale<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image(
                "canvas/filter/greyscale",
                &[("avatar", avatar_url.to_string())],
            )
            .await
    }

    /// Invert and grayscale your avatar
    pub async fn invert_greyscale<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image(
                "canvas/filter/invertgreyscale",
                &[("avatar", avatar_url.to_string())],
            )
            .await
    }

    /// Redify(?) your avatar
    pub async fn red<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image("canvas/filter/red", &[("avatar", avatar_url.to_string())])
            .await
    }

    /// Apply a sepia filter to your avatar
    pub async fn sepia<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image("canvas/filter/sepia", &[("avatar", avatar_url.to_string())])
            .await
    }

    /// Apply threshold to your avatar
    pub async fn threshold<T: ToString>(&self, avatar_url: T, threshold: u8) -> Result<Vec<u8>> {
        self.0
            .request_image(
                "canvas/filter/threshold",
                &[
                    ("avatar", avatar_url.to_string()),
                    ("threshold", threshold.min(100).to_string()),
                ],
            )
            .await
    }
}
