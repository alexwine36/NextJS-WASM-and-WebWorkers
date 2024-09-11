use wasm_bindgen::prelude::*;
pub static COLORS: [(&str, &str); 6] = [
    ("Black", "#000000"),
    ("Green", "#3DC06C"),
    ("Red", "#FF0000"),
    ("Blue", "#4F8DE4"),
    ("Yellow", "#FAE589"),
    ("White", "#FFFFFF"),
];

static DEFAULT_COLOR: &str = COLORS[0].1;
use crate::log;
use crate::settings::Settings;
use utilities::console_log;
use web_sys::{console, window, Element, HtmlCanvasElement, HtmlElement, MouseEvent};
pub static PEN_SIZES: [f64; 4] = [1.0, 2.0, 4.0, 8.0];

static DEFAULT_PEN_SIZE: f64 = PEN_SIZES[0];

#[derive(Debug)]
struct Dimensions {
    width: u32,
    height: u32,
    x: f64,
    y: f64,
}

fn get_dimensions(body: &HtmlElement) -> Dimensions {
    // let client_width = body.client_width() as u32;
    // let client_height = body.client_height() as u32;

    // let width = min(max(client_width, 600), 3000);
    // let height = min(max(client_height, 400), 2000);
    let rect = body.get_bounding_client_rect();

    Dimensions {
        width: rect.width() as u32,
        height: rect.height() as u32,
        x: rect.left(),
        y: rect.top(),
    }
    // (width, height)
}

#[derive(Debug)]
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
    pub settings: Settings,
}

impl State {
    pub fn new(canvas_el: &HtmlCanvasElement, settings: Settings) -> State {
        let dimensions = get_dimensions(&canvas_el);
        console_log!("dimensions: {:?}", dimensions);
        State {
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
        }
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

    pub fn get_mouse_position(&self, event: MouseEvent) -> (f64, f64) {
        let x = event.client_x() as f64 - self.get_x();
        let y = event.client_y() as f64 - self.get_y();
        // console_log!("event: {:?}", event.type_());
        // console_log!("x: {}, y: {}", x, y);
        (x, y)
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
