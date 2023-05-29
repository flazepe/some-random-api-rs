pub mod filter;
pub mod misc;
pub mod overlay;
pub mod structs;

use filter::CanvasFilterEndpoint;
use misc::CanvasMiscEndpoint;
use overlay::CanvasOverlayEndpoint;

pub struct CanvasEndpoint {
    pub filter: CanvasFilterEndpoint,
    pub misc: CanvasMiscEndpoint,
    pub overlay: CanvasOverlayEndpoint,
}
