use crate::{Image, Requester};
use anyhow::Result;

pub struct ImageEndpoint(pub(crate) Requester);

impl ImageEndpoint {
    /// An endpoint that sends a random image of a bird
    pub async fn bird(&self) -> Result<Image> {
        self.0.request("img/bird").await
    }

    /// An endpoint that sends a random image of a cat
    pub async fn cat(&self) -> Result<Image> {
        self.0.request("img/cat").await
    }

    /// An endpoint that sends a random image of a dog
    pub async fn dog(&self) -> Result<Image> {
        self.0.request("img/dog").await
    }

    /// An endpoint that sends a random image of a fox
    pub async fn fox(&self) -> Result<Image> {
        self.0.request("img/fox").await
    }

    /// An endpoint that sends a random image of a kangaroo
    pub async fn kangaroo(&self) -> Result<Image> {
        self.0.request("img/fox").await
    }

    /// An endpoint that sends a random image of a koala
    pub async fn koala(&self) -> Result<Image> {
        self.0.request("img/koala").await
    }

    /// An endpoint that sends a random image of a panda
    pub async fn panda(&self) -> Result<Image> {
        self.0.request("img/panda").await
    }

    /// An endpoint that sends a random image of a pikachu
    pub async fn pikachu(&self) -> Result<Image> {
        self.0.request("img/pikachu").await
    }

    /// An endpoint that sends a random image of a raccoon
    pub async fn raccoon(&self) -> Result<Image> {
        self.0.request("img/raccoon").await
    }

    /// An endpoint that sends a random image of a red panda
    pub async fn red_panda(&self) -> Result<Image> {
        self.0.request("img/red_panda").await
    }

    /// An endpoint that sends a random image of a whale
    pub async fn whale(&self) -> Result<Image> {
        self.0.request("img/whale").await
    }
}
