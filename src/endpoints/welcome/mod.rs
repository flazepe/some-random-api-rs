use crate::{requester::Requester, WelcomeImage};
use anyhow::Result;

pub struct WelcomeEndpoint(pub(crate) Requester);

impl WelcomeEndpoint {
    /// Welcome image
    pub async fn image(&self, welcome_image: WelcomeImage) -> Result<Vec<u8>> {
        self.0
            .request_image(
                format!(
                    "welcome/img/{}/{}",
                    welcome_image.template,
                    format!("{:?}", welcome_image.background).to_lowercase(),
                ),
                &welcome_image.to_query(),
            )
            .await
    }
}
