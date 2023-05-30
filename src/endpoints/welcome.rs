use crate::{Requester, WelcomeImage};
use anyhow::Result;

pub struct WelcomeEndpoint(pub(crate) Requester);

impl WelcomeEndpoint {
    /// Generate a free welcome image
    pub async fn image(&self, welcome_image: WelcomeImage) -> Result<Vec<u8>> {
        self.0
            .request_image(
                format!(
                    "welcome/img/{}/{}",
                    welcome_image.template,
                    format!("{:?}", welcome_image.background).to_lowercase(),
                ),
                &welcome_image.into_query(),
            )
            .await
    }
}
