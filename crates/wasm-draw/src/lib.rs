extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
pub mod app;
mod settings;
mod state;
pub mod tool;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

}

#[wasm_bindgen]
pub fn init_app(canvas_el: HtmlCanvasElement) -> Result<(), JsValue> {
    let mut app = app::App::new(canvas_el);

    app.run();

    Ok(())
}
