extern crate wasm_bindgen;

use crate::log;
use crate::state::State;
// use crate::tool::Line;
// use crate::tool::ToolBelt;
use std::cell::RefCell;
use std::rc::Rc;
use utilities::console_log;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};
pub fn init_canvas(canvas: &HtmlCanvasElement, state: &Rc<RefCell<State>>) -> Result<(), JsValue> {
    canvas.set_width(state.borrow().get_width());
    canvas.set_height(state.borrow().get_height());

    // let toolbelt = ToolBelt::new(canvas, state);
    // toolbelt.init();

    Ok(())
}
