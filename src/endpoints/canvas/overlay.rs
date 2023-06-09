use crate::Requester;
use anyhow::Result;

pub struct CanvasOverlayEndpoint(pub(crate) Requester);

impl CanvasOverlayEndpoint {
    /// Give your avatar a comrade overlay
    pub async fn comrade<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image(
                "canvas/overlay/comrade",
                &[("avatar", avatar_url.to_string())],
            )
            .await
    }

    /// Give your avatar a rainbow overlay
    pub async fn gay<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image("canvas/overlay/gay", &[("avatar", avatar_url.to_string())])
            .await
    }

    /// Give your avatar a glass effect overlay
    pub async fn glass<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image(
                "canvas/overlay/glass",
                &[("avatar", avatar_url.to_string())],
            )
            .await
    }

    /// Go to horny jail
    pub async fn jail<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image("canvas/overlay/jail", &[("avatar", avatar_url.to_string())])
            .await
    }

    /// Mission passed
    pub async fn passed<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image(
                "canvas/overlay/passed",
                &[("avatar", avatar_url.to_string())],
            )
            .await
    }

    /// I will not enter a description so you will get triggered
    pub async fn triggered<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image(
                "canvas/overlay/triggered",
                &[("avatar", avatar_url.to_string())],
            )
            .await
    }

    /// Wastedeeznuts
    pub async fn wasted<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image(
                "canvas/overlay/wasted",
                &[("avatar", avatar_url.to_string())],
            )
            .await
    }
}
