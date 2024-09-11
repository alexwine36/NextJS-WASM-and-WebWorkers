extern crate wasm_bindgen;

use std::cell::RefCell;
use std::cmp::{max, min};
use std::rc::Rc;
use utilities::console_log;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Element, HtmlCanvasElement, HtmlElement};
mod canvas;

mod draw_engine;
mod settings;
mod state;
mod tool;
mod toolbar;

static TOOLBAR_WIDTH: u32 = 50;

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
    let window = window().expect("Could not find `window`");
    let document = window.document().expect("Could not find `document`");
    let body = document.body().expect("Could not find `body` element");

    // let (w, h) = get_dimensions(&canvas_el);
    // let w = canvas_el.width();
    // let h = canvas_el.height();

    let mut settings = settings::Settings::new();
    console_log!(
        "colors {:?}, pen sizes {:?}",
        settings.colors,
        settings.pen_sizes
    );
    settings.add_color("something", "#aa00bb");
    let state: Rc<RefCell<state::State>> =
        Rc::new(RefCell::new(state::State::new(&canvas_el, settings)));
    console_log!("state: {:?}", state);

    // console_log!("width: {:?}, height: {:?}", state.);
    // let root = document.create_element("div")?;
    // root.set_attribute("style", "min-height: 100%;");

    // body.append_child(&root);

    // let canvas_el = document
    //     .create_element("canvas")?
    //     .dyn_into::<HtmlCanvasElement>()?;
    // canvas_el.set_width(w - TOOLBAR_WIDTH);
    // canvas_el.set_height(h);
    let rect = canvas_el.get_bounding_client_rect();
    console_log!("rect: {:?}", rect.y());
    // root.append_child(&canvas_el)?;
    canvas::init_canvas(&canvas_el, &state)?;

    let toolbar_el = document.create_element("div")?.dyn_into::<Element>()?;
    toolbar_el.set_attribute(
        "style",
        "width:100%; border-left: 1px solid #efefef; display: flex;",
    )?;
    let parent = canvas_el
        .parent_element()
        .expect("Could not find parent element");
    console_log!("parent: {:?}", parent);
    body.append_child(&toolbar_el)?;
    // body.append_child(&toolbar_el)?;
    toolbar::init_toolbar(&toolbar_el, &canvas_el, &state);

    Ok(())
}
