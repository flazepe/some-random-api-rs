use crate::{AnimuQuote, Image, Requester};
use anyhow::Result;

pub struct AnimuEndpoint(pub(crate) Requester);

impl AnimuEndpoint {
    /// Facepalm
    pub async fn facepalm(&self) -> Result<Image> {
        self.0.request("animu/face-palm", None::<&()>).await
    }

    /// Hug
    pub async fn hug(&self) -> Result<Image> {
        self.0.request("animu/hug", None::<&()>).await
    }

    /// Pat pat
    pub async fn pat(&self) -> Result<Image> {
        self.0.request("animu/pat", None::<&()>).await
    }

    /// Random anime quote
    pub async fn quote(&self) -> Result<AnimuQuote> {
        self.0.request("animu/quote", None::<&()>).await
    }

    /// Wink wink
    pub async fn wink(&self) -> Result<Image> {
        self.0.request("animu/wink", None::<&()>).await
    }
}
