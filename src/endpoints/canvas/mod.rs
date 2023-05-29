pub mod filter;
pub mod misc;
pub mod overlay;

pub use filter::CanvasFilterEndpoint;
pub use misc::CanvasMiscEndpoint;
pub use overlay::CanvasOverlayEndpoint;

pub struct CanvasEndpoint {
    pub filter: CanvasFilterEndpoint,
    pub misc: CanvasMiscEndpoint,
    pub overlay: CanvasOverlayEndpoint,
}
