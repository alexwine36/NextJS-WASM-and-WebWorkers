// mod lib;

use wasm_bindgen::JsCast;
use wasm_draw::init_app;
// mod lib;
use web_sys::{window, Element, HtmlCanvasElement, HtmlElement};

fn main() {
    console_error_panic_hook::set_once();

    let document = window()
        .and_then(|win| win.document())
        .expect("Could not access the document");
    let body = document.body().expect("Could not access document.body");
    let text_el = document.create_element("p").unwrap();

    text_el.set_text_content("Hello, world from Vanilla Rust!".into());
    body.append_child(text_el.as_ref())
        .expect("Failed to append text");

    let canvas_el: HtmlCanvasElement = document
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    canvas_el
        .set_attribute(
            "style",
            "margin: 50px; width: 50vw; height: 50vh; border: 1px solid black;",
        )
        .unwrap();
    body.append_child(&canvas_el)
        .expect("Failed to append canvas");

    init_app(canvas_el).expect("Failed to initialize app");
}
