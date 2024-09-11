use std::collections::HashMap;

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
use js_sys::Array;
use serde::{Deserialize, Serialize};
use utilities::console_log;
use web_sys::{console, js_sys, window, Element, HtmlCanvasElement, HtmlElement, MouseEvent};
pub static PEN_SIZES: [f64; 4] = [1.0, 2.0, 4.0, 8.0];

static DEFAULT_PEN_SIZE: f64 = PEN_SIZES[0];

#[derive(Debug, Serialize, Deserialize, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct Color {
    pub name: String,
    pub hex: String,
}

impl Color {
    pub fn new(name: &str, hex: &str) -> Self {
        Self {
            name: name.to_string(),
            hex: hex.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
// #[wasm_bindgen]
#[wasm_bindgen(getter_with_clone)]
pub struct Settings {
    pub colors: Vec<Color>,

    pub pen_sizes: Vec<f64>,

    pub tools: Vec<String>,
}

#[wasm_bindgen]
impl Settings {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let colors = COLORS
            .iter()
            .map(|(name, hex)| Color {
                name: name.to_string(),
                hex: hex.to_string(),
            })
            .collect();

        Self {
            colors,
            pen_sizes: PEN_SIZES.to_vec(),
            tools: vec!["pen".to_string(), "line".to_string(), "circle".to_string()],
        }
    }

    pub fn add_color(&mut self, name: &str, hex: &str) {
        self.colors.push(Color::new(name, hex));
    }

    pub fn get_colors(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.colors).unwrap()
    }

    pub fn get_pen_sizes(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.pen_sizes).unwrap()
    }
}
