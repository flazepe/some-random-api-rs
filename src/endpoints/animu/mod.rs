pub mod structs;

use crate::{image::structs::Image, requester::Requester};
use anyhow::Result;
use structs::Quote;

pub struct AnimuEndpoint(pub(crate) Requester);

impl AnimuEndpoint {
    /// Facepalm
    pub async fn facepalm(&self) -> Result<Image> {
        self.0.request("animu/face-palm").await
    }

    /// Hug
    pub async fn hug(&self) -> Result<Image> {
        self.0.request("animu/hug").await
    }

    /// Pat pat
    pub async fn pat(&self) -> Result<Image> {
        self.0.request("animu/pat").await
    }

    /// Random anime quote
    pub async fn quote(&self) -> Result<Quote> {
        self.0.request("animu/quote").await
    }

    /// Wink wink
    pub async fn wink(&self) -> Result<Image> {
        self.0.request("animu/wink").await
    }
}
