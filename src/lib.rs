pub mod keywords;
pub mod transpiler;

#[cfg(not(target_arch = "wasm32"))]
pub mod explain;
#[cfg(not(target_arch = "wasm32"))]
pub mod kibble;
#[cfg(not(target_arch = "wasm32"))]
pub mod logo;

#[cfg(feature = "wasm")]
pub mod wasm;
