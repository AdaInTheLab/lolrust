use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn transpile_meow(source: &str) -> String {
    crate::transpiler::transpile(source)
}
