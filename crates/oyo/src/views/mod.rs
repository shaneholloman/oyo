//! View rendering modules

mod evolution;
mod split;
mod single_pane;

pub use evolution::render_evolution;
pub use split::render_split;
pub use single_pane::render_single_pane;
