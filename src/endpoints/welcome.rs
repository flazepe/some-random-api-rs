use crate::{Requester, WelcomeImage};
use anyhow::Result;
use serde_json::to_string;

/// An endpoint for generating free welcome images
///
/// # Examples
///
/// ```
/// use some_random_api::{Client, WelcomeImage, WelcomeImageBackground, WelcomeImageTextColor};
/// use std::fs::write;
///
/// write(
///     "welcome.png",
///     Client::new(None::<String>)
///         .welcome
///         .image(
///             WelcomeImage::new(
///                 "username",
///                 "avatar url",
///                 "guild name",
///                 100, // Member count
///             )
///             .set_background(WelcomeImageBackground::Stars)
///             .set_text_color(WelcomeImageTextColor::Red)
///             .set_leave(true);
///         ).await?,
/// )?;
/// ```
pub struct WelcomeEndpoint(pub(crate) Requester);

impl WelcomeEndpoint {
    /// Generate a free welcome image
    pub async fn image(&self, welcome_image: WelcomeImage) -> Result<Vec<u8>> {
        self.0
            .request_image(
                format!(
                    "welcome/img/{}/{}",
                    to_string(&welcome_image.template)?,
                    format!("{:?}", welcome_image.background).to_lowercase(),
                ),
                &welcome_image,
            )
            .await
    }
}
