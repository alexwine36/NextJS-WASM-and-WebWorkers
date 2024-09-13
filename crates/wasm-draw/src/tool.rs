use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[wasm_bindgen]
pub enum ToolType {
    Pen,
    Line,
    Rectangle,
    // Circle,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct Measurement {
    pub points: Vec<Point>,
    color: String,
    pen_size: f64,
    tool: ToolType,
}

impl Measurement {
    pub fn new(color: &String, pen_size: f64, tool: ToolType) -> Self {
        Self {
            points: Vec::new(),
            color: color.into(),
            pen_size,
            tool,
        }
    }

    pub fn add_point(&mut self, x: f64, y: f64) {
        self.points.push(Point::new(x, y));
    }

    pub fn finish(&mut self) {
        if self.tool != ToolType::Pen && self.points.len() > 2 {
            let last_point = self.points.last().unwrap();
            let first_point = self.points.first().unwrap();
            self.points = vec![first_point.clone(), last_point.clone()];
        }
    }

    pub fn draw(&self, context: &CanvasRenderingContext2d) {
        // console_log!("drawing measurement {:?}", self);
        if self.points.len() < 2 {
            return;
        }
        context.set_stroke_style(&JsValue::from_str(&self.color));
        context.set_line_width(self.pen_size);
        context.begin_path();
        let first_point = self.points.first().unwrap();
        context.move_to(first_point.x, first_point.y);
        match self.tool {
            ToolType::Pen => {
                for point in self.points.iter().skip(1) {
                    context.line_to(point.x, point.y);
                }
            }
            ToolType::Line => {
                let last_point = self.points.last().unwrap();
                context.line_to(last_point.x, last_point.y);
            }
            ToolType::Rectangle => {
                let last_point = self.points.last().unwrap();
                let first_point = self.points.first().unwrap();
                context.rect(
                    first_point.x,
                    first_point.y,
                    last_point.x - first_point.x,
                    last_point.y - first_point.y,
                );
            }
        }

        context.stroke();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn it_works() {
        let mut measurement = Measurement::new(&"red".to_string(), 2.0, ToolType::Rectangle);

        for _ in 0..100 {
            let x = rand::thread_rng().gen_range(0.0..100.0);
            let y = rand::thread_rng().gen_range(0.0..100.0);
            measurement.add_point(x, y);
        }
        assert_eq!(measurement.points.len(), 100);
        measurement.finish();
        assert_eq!(measurement.points.len(), 2);
    }
}
