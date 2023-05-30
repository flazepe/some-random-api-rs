use crate::{RankCard, Requester, WelcomeImage};
use anyhow::Result;

pub struct PremiumEndpoint(pub(crate) Requester);

impl PremiumEndpoint {
    /// Sussy
    pub async fn among_us<T: ToString, U: ToString, V: ToString>(
        &self,
        username: T,
        avatar_url: U,
        text: Option<V>,
    ) -> Result<Vec<u8>> {
        self.0
            .request_image(
                "premium/amongus",
                &[
                    ("username", username.to_string()),
                    ("avatar", avatar_url.to_string()),
                    ("custom", text.map_or("".into(), |text| text.to_string())),
                ],
            )
            .await
    }

    // Petpet
    pub async fn petpet<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image("premium/petpet", &[("avatar", avatar_url.to_string())])
            .await
    }

    // Rank card
    pub async fn rank_card(&self, rank_card: RankCard) -> Result<Vec<u8>> {
        self.0
            .request_image(
                format!("premium/rankcard/{}", rank_card.template),
                &rank_card.into_query(),
            )
            .await
    }

    // Welcome image
    pub async fn welcome(&self, welcome_image: WelcomeImage) -> Result<Vec<u8>> {
        self.0
            .request_image(
                format!("premium/welcome/{}", welcome_image.template),
                &welcome_image.into_query(),
            )
            .await
    }
}
