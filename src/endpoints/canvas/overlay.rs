use crate::Requester;
use anyhow::Result;

pub struct CanvasOverlayEndpoint(pub(crate) Requester);

impl CanvasOverlayEndpoint {
    pub async fn comrade<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image(
                "canvas/overlay/comrade",
                &[("avatar", avatar_url.to_string())],
            )
            .await
    }

    pub async fn gay<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image("canvas/overlay/gay", &[("avatar", avatar_url.to_string())])
            .await
    }

    pub async fn glass<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image(
                "canvas/overlay/glass",
                &[("avatar", avatar_url.to_string())],
            )
            .await
    }

    pub async fn jail<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image("canvas/overlay/jail", &[("avatar", avatar_url.to_string())])
            .await
    }

    pub async fn passed<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image(
                "canvas/overlay/passed",
                &[("avatar", avatar_url.to_string())],
            )
            .await
    }

    pub async fn triggered<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image(
                "canvas/overlay/triggered",
                &[("avatar", avatar_url.to_string())],
            )
            .await
    }

    pub async fn wasted<T: ToString>(&self, avatar_url: T) -> Result<Vec<u8>> {
        self.0
            .request_image(
                "canvas/overlay/wasted",
                &[("avatar", avatar_url.to_string())],
            )
            .await
    }
}
