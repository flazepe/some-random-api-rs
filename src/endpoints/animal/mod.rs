pub mod structs;

use crate::requester::Requester;
use anyhow::Result;
use structs::Animal;

pub struct AnimalEndpoint(pub(crate) Requester);

impl AnimalEndpoint {
    /// An endpoint that sends a random image and fact of a bird
    pub async fn bird(&self) -> Result<Animal> {
        self.0.request("animal/bird").await
    }

    /// An endpoint that sends a random image and fact of a cat
    pub async fn cat(&self) -> Result<Animal> {
        self.0.request("animal/cat").await
    }

    /// An endpoint that sends a random image and fact of a dog
    pub async fn dog(&self) -> Result<Animal> {
        self.0.request("animal/dog").await
    }

    /// An endpoint that sends a random image and fact of a fox
    pub async fn fox(&self) -> Result<Animal> {
        self.0.request("animal/fox").await
    }

    /// An endpoint that sends a random image and fact of a kangaroo
    pub async fn kangaroo(&self) -> Result<Animal> {
        self.0.request("animal/kangaroo").await
    }

    /// An endpoint that sends a random image and fact of a koala
    pub async fn koala(&self) -> Result<Animal> {
        self.0.request("animal/koala").await
    }

    /// An endpoint that sends a random image and fact of a panda
    pub async fn panda(&self) -> Result<Animal> {
        self.0.request("animal/panda").await
    }

    /// An endpoint that sends a random image and fact of a raccoon
    pub async fn raccoon(&self) -> Result<Animal> {
        self.0.request("animal/raccoon").await
    }

    /// An endpoint that sends a random image and fact of a red panda
    pub async fn red_panda(&self) -> Result<Animal> {
        self.0.request("animal/red_panda").await
    }
}
