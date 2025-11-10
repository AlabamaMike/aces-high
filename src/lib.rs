//! ACES HIGH: ENDLESS SKIES
//!
//! A WebAssembly-based roguelike arcade shooter featuring HD graphics,
//! procedural generation, and meta-progression systems.

use wasm_bindgen::prelude::*;

pub mod engine;
pub mod utils;
pub mod game;
pub mod web;

// Re-exports for convenience
// pub use engine::renderer::Renderer;
pub use game::components::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen(start)]
pub fn start() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    console_log!("ACES HIGH: WebAssembly module loaded");
}
