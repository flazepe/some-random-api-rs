use crate::{Fact, Requester};
use anyhow::Result;

pub struct FactEndpoint(pub(crate) Requester);

impl FactEndpoint {
    /// An endpoint that sends a random bird fact
    pub async fn bird(&self) -> Result<Fact> {
        self.0.request("facts/bird", None::<&()>).await
    }

    /// An endpoint that sends a random cat fact
    pub async fn cat(&self) -> Result<Fact> {
        self.0.request("facts/cat", None::<&()>).await
    }

    /// An endpoint that sends a random dog fact
    pub async fn dog(&self) -> Result<Fact> {
        self.0.request("facts/dog", None::<&()>).await
    }

    /// An endpoint that sends a random fox fact
    pub async fn fox(&self) -> Result<Fact> {
        self.0.request("facts/fox", None::<&()>).await
    }

    /// An endpoint that sends a random koala fact
    pub async fn koala(&self) -> Result<Fact> {
        self.0.request("facts/koala", None::<&()>).await
    }

    /// An endpoint that sends a random panda fact
    pub async fn panda(&self) -> Result<Fact> {
        self.0.request("facts/panda", None::<&()>).await
    }
}
