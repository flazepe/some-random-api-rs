pub mod structs;

use crate::requester::Requester;
use anyhow::Result;
use structs::Image;

pub struct ImageEndpoint(pub(crate) Requester);

impl ImageEndpoint {
    pub async fn bird(&self) -> Result<Image> {
        self.0.request("img/bird").await
    }

    pub async fn cat(&self) -> Result<Image> {
        self.0.request("img/cat").await
    }

    pub async fn dog(&self) -> Result<Image> {
        self.0.request("img/dog").await
    }

    pub async fn fox(&self) -> Result<Image> {
        self.0.request("img/fox").await
    }

    pub async fn kangaroo(&self) -> Result<Image> {
        self.0.request("img/fox").await
    }

    pub async fn koala(&self) -> Result<Image> {
        self.0.request("img/koala").await
    }

    pub async fn panda(&self) -> Result<Image> {
        self.0.request("img/panda").await
    }

    pub async fn pikachu(&self) -> Result<Image> {
        self.0.request("img/pikachu").await
    }

    pub async fn raccoon(&self) -> Result<Image> {
        self.0.request("img/raccoon").await
    }

    pub async fn red_panda(&self) -> Result<Image> {
        self.0.request("img/red_panda").await
    }

    pub async fn whale(&self) -> Result<Image> {
        self.0.request("img/whale").await
    }
}
