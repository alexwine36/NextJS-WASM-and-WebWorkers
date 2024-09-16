extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlCanvasElement};
pub mod app;
mod settings;
mod state;
pub mod tool;
mod toolbar;

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

    let mut app = app::App::new(canvas_el);
    // let state = app.get_dimensions();
    // let measurement = app.start_drawing();
    // let mut measurement = app.get_measurement();
    // for _ in 0..100 {
    //     let x = rand::thread_rng().gen_range(0.0..state.width.into());
    //     let y = rand::thread_rng().gen_range(0.0..state.height.into());
    //     measurement.borrow_mut().add_point(x, y);
    // }
    // console_log!("state: {:?}", state);

    app.run();

    // let mut settings = settings::Settings::new();
    // console_log!(
    //     "colors {:?}, pen sizes {:?}",
    //     settings.colors,
    //     settings.pen_sizes
    // );
    // settings.add_color("something", "#aa00bb");
    // let state: Rc<RefCell<state::State>> =
    //     Rc::new(RefCell::new(state::State::new(&canvas_el, settings)));
    // console_log!("state: {:?}", state);

    // // console_log!("width: {:?}, height: {:?}", state.);
    // // let root = document.create_element("div")?;
    // // root.set_attribute("style", "min-height: 100%;");

    // // body.append_child(&root);

    // // let canvas_el = document
    // //     .create_element("canvas")?
    // //     .dyn_into::<HtmlCanvasElement>()?;
    // // canvas_el.set_width(w - TOOLBAR_WIDTH);
    // // canvas_el.set_height(h);
    // let rect = canvas_el.get_bounding_client_rect();
    // console_log!("rect: {:?}", rect.y());
    // // root.append_child(&canvas_el)?;

    // canvas::init_canvas(&canvas_el, &state)?;

    // let toolbar_el = document.create_element("div")?.dyn_into::<Element>()?;
    // toolbar_el.set_attribute(
    //     "style",
    //     "width:100%; border-left: 1px solid #efefef; display: flex;",
    // )?;
    // let parent = canvas_el
    //     .parent_element()
    //     .expect("Could not find parent element");
    // console_log!("parent: {:?}", parent);
    // body.append_child(&toolbar_el)?;

    // toolbar::init_toolbar(&toolbar_el, &canvas_el, &state);

    Ok(())
}
