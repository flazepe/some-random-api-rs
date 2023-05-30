mod filter;
mod misc;
mod overlay;

pub use filter::CanvasFilterEndpoint;
pub use misc::CanvasMiscEndpoint;
pub use overlay::CanvasOverlayEndpoint;

/// An endpoint for generating images manipulated using Canvas
///
/// # Examples
///
/// ```
/// use some_random_api::Client;
/// use std::fs::write;
///
/// write(
///     "jail.png",
///     Client::new(None::<String>)
///         .canvas
///         .overlay
///         .jail("avatar url").await?,
/// )?;
/// ```
pub struct CanvasEndpoint {
    pub filter: CanvasFilterEndpoint,
    pub misc: CanvasMiscEndpoint,
    pub overlay: CanvasOverlayEndpoint,
}
