use crate::{Animal, Requester};
use anyhow::Result;

pub struct AnimalEndpoint(pub(crate) Requester);

impl AnimalEndpoint {
    /// An endpoint that sends a random image and fact of a bird
    pub async fn bird(&self) -> Result<Animal> {
        self.0.request("animal/bird", None::<&()>).await
    }

    /// An endpoint that sends a random image and fact of a cat
    pub async fn cat(&self) -> Result<Animal> {
        self.0.request("animal/cat", None::<&()>).await
    }

    /// An endpoint that sends a random image and fact of a dog
    pub async fn dog(&self) -> Result<Animal> {
        self.0.request("animal/dog", None::<&()>).await
    }

    /// An endpoint that sends a random image and fact of a fox
    pub async fn fox(&self) -> Result<Animal> {
        self.0.request("animal/fox", None::<&()>).await
    }

    /// An endpoint that sends a random image and fact of a kangaroo
    pub async fn kangaroo(&self) -> Result<Animal> {
        self.0.request("animal/kangaroo", None::<&()>).await
    }

    /// An endpoint that sends a random image and fact of a koala
    pub async fn koala(&self) -> Result<Animal> {
        self.0.request("animal/koala", None::<&()>).await
    }

    /// An endpoint that sends a random image and fact of a panda
    pub async fn panda(&self) -> Result<Animal> {
        self.0.request("animal/panda", None::<&()>).await
    }

    /// An endpoint that sends a random image and fact of a raccoon
    pub async fn raccoon(&self) -> Result<Animal> {
        self.0.request("animal/raccoon", None::<&()>).await
    }

    /// An endpoint that sends a random image and fact of a red panda
    pub async fn red_panda(&self) -> Result<Animal> {
        self.0.request("animal/red_panda", None::<&()>).await
    }
}
