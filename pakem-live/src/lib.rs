pub mod component;
pub mod directives;
pub mod extension;
pub mod prelude;
pub mod registry;
pub mod snapshot;
pub mod transport;

pub use component::{LiveComponent, parse_blade_args_to_json};
pub use extension::LiveApplicationExt;
pub use registry::{ComponentHandler, LiveComponentHandler, LiveRegistry};
pub use snapshot::{LiveSnapshot, SnapshotManager};
pub use transport::{call_action_handler, live_ws_handler, serve_live_js, LiveCallRequest, LiveCallResponse};

/// Inisialisasi otomatis direktif Pakem Live ke dalam Blade Engine.
pub fn init() {
    directives::register_live_directives();
}

/// Menghasilkan Axum router berisi endpoint Pakem Live:
/// - `GET  /_pakem/live/live.js`
/// - `POST /_pakem/live/call`
/// - `GET  /_pakem/live/ws`
pub fn routes() -> axum::Router<pakem::prelude::AppState> {
    transport::live_routes()
}
