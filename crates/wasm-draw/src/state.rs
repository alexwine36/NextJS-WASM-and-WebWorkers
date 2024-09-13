use wasm_bindgen::prelude::*;
pub static COLORS: [(&str, &str); 6] = [
    ("Black", "#000000"),
    ("Green", "#3DC06C"),
    ("Red", "#FF0000"),
    ("Blue", "#4F8DE4"),
    ("Yellow", "#FAE589"),
    ("White", "#FFFFFF"),
];
use serde::{Deserialize, Serialize};
static DEFAULT_COLOR: &str = COLORS[0].1;
use crate::settings::Settings;
use crate::{log, tool::ToolType};
use utilities::console_log;
use web_sys::{console, window, Element, HtmlCanvasElement, HtmlElement, MouseEvent};
pub static PEN_SIZES: [f64; 4] = [1.0, 2.0, 4.0, 8.0];

static DEFAULT_PEN_SIZE: f64 = PEN_SIZES[0];

#[derive(Debug, Serialize, Deserialize, Clone)]
#[wasm_bindgen]
pub struct Dimensions {
    pub width: u32,
    pub height: u32,
    pub x: f64,
    pub y: f64,
}

pub fn get_element_dimensions(body: &HtmlElement) -> Dimensions {
    let rect = body.get_bounding_client_rect();

    Dimensions {
        width: rect.width() as u32,
        height: rect.height() as u32,
        x: rect.left(),
        y: rect.top(),
    }
}
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct State {
    width: u32,
    height: u32,
    x: f64,
    y: f64,
    is_drawing: bool,
    color: String,
    pen_size: f64,
    undo_list: Vec<String>,
    redo_list: Vec<String>,
    dimensions: Dimensions,
    settings: Settings,
    tool_type: ToolType,
}

#[wasm_bindgen]
impl State {
    pub fn new(canvas_el: &HtmlCanvasElement, settings: Settings) -> State {
        let dimensions = get_element_dimensions(&canvas_el);
        console_log!("dimensions: {:?}", dimensions);
        State {
            dimensions: dimensions.clone(),
            width: dimensions.width,
            height: dimensions.height,
            x: dimensions.x,
            y: dimensions.y,
            is_drawing: false,
            color: DEFAULT_COLOR.to_string(),
            pen_size: DEFAULT_PEN_SIZE,
            undo_list: vec![],
            redo_list: vec![],
            settings,
            tool_type: ToolType::Rectangle,
        }
    }

    pub fn get_tool(&self) -> ToolType {
        self.tool_type.clone()
    }

    pub fn set_tool(&mut self, tool: ToolType) {
        self.tool_type = tool;
    }

    pub fn start_drawing(&mut self) {
        self.redo_list = vec![];
        self.is_drawing = true;
    }

    pub fn stop_drawing(&mut self) {
        self.is_drawing = false;
    }

    pub fn is_drawing(&self) -> bool {
        self.is_drawing
    }

    pub fn update_color(&mut self, color: String) {
        self.color = color;
    }

    pub fn get_color(&self) -> String {
        self.color.clone()
    }

    pub fn update_pen_size(&mut self, size: f64) {
        self.pen_size = size;
    }

    pub fn get_pen_size(&self) -> f64 {
        self.pen_size
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn get_dimensions(&self) -> Dimensions {
        self.dimensions.clone()
    }

    pub fn set_dimensions(&mut self, dimensions: Dimensions) {
        self.dimensions = dimensions.clone();
        self.width = dimensions.width;
        self.height = dimensions.height;
        self.x = dimensions.x;
        self.y = dimensions.y;
    }

    pub fn get_settings(&self) -> Settings {
        self.settings.clone()
    }

    pub fn add_undo_state(&mut self, prev: String) {
        // console_log!("prev: {:?}", prev);
        self.undo_list.push(prev);
    }

    pub fn add_redo_state(&mut self, next: String) {
        self.redo_list.push(next);
    }

    pub fn undo(&mut self) -> Option<String> {
        self.undo_list.pop()
    }

    pub fn redo(&mut self) -> Option<String> {
        self.redo_list.pop()
    }
}

impl State {
    pub fn get_mouse_position(&self, event: MouseEvent) -> (f64, f64) {
        let x = event.client_x() as f64 - self.get_x();
        let y = event.client_y() as f64 - self.get_y();
        // console_log!("event: {:?}", event.type_());
        // console_log!("x: {}, y: {}", x, y);
        (x, y)
    }
}
