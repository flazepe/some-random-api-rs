use crate::{AnimuQuote, Image, Requester};
use anyhow::Result;

pub struct AnimuEndpoint(pub(crate) Requester);

impl AnimuEndpoint {
    /// Generate a random anime facepalm GIF
    pub async fn facepalm(&self) -> Result<Image> {
        self.0.request("animu/face-palm", None::<&()>).await
    }

    /// Generate a random anime hug GIF
    pub async fn hug(&self) -> Result<Image> {
        self.0.request("animu/hug", None::<&()>).await
    }

    /// Generate a random anime pat GIF
    pub async fn pat(&self) -> Result<Image> {
        self.0.request("animu/pat", None::<&()>).await
    }

    /// Generate a random anime quote
    pub async fn quote(&self) -> Result<AnimuQuote> {
        self.0.request("animu/quote", None::<&()>).await
    }

    /// Generate a random anime wink GIF
    pub async fn wink(&self) -> Result<Image> {
        self.0.request("animu/wink", None::<&()>).await
    }
}
