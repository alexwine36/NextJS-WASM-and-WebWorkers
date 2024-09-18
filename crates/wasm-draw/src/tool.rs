use std::f64::consts::PI;

use crate::log;
use crate::state::get_element_dimensions;
use serde::{Deserialize, Serialize};
use utilities::console_log;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, ImageData};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[wasm_bindgen]
pub enum ToolType {
    Pen,
    Line,
    Rectangle,
    Circle,
    Fill,
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[wasm_bindgen(getter_with_clone)]
struct SimplePoint {
    x: u32,
    y: u32,
}

impl SimplePoint {
    pub fn to_tuple(&self) -> (u32, u32) {
        (self.x, self.y)
    }
}

impl From<Point> for SimplePoint {
    fn from(val: Point) -> Self {
        SimplePoint {
            x: val.x as u32,
            y: val.y as u32,
        }
    }
}

impl From<SimplePoint> for (u32, u32) {
    fn from(point: SimplePoint) -> Self {
        (point.x, point.y)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct Measurement {
    pub points: Vec<Point>,
    color: String,
    pen_size: f64,
    tool: ToolType,
    completed: bool,
    processed: bool,
}

impl Measurement {
    pub fn new(color: &String, pen_size: f64, tool: ToolType) -> Self {
        Self {
            points: Vec::new(),
            color: color.into(),
            pen_size,
            tool,
            completed: false,
            processed: false,
        }
    }

    pub fn add_point(&mut self, x: f64, y: f64) {
        self.points.push(Point::new(x, y));
    }

    pub fn finish(&mut self) {
        if self.tool != ToolType::Pen && self.tool != ToolType::Fill && self.points.len() > 2 {
            let last_point = self.points.last().unwrap();
            let first_point = self.points.first().unwrap();
            self.points = vec![first_point.clone(), last_point.clone()];
        }
        if self.tool == ToolType::Fill {
            // todo!()
        }
        self.completed = true;
    }

    fn draw_quadratic_curve(&self, context: &CanvasRenderingContext2d) {
        context.set_stroke_style(&JsValue::from_str(&self.color));
        context.set_fill_style(&JsValue::from_str(&self.color));
        context.set_line_width(self.pen_size);
        context.set_line_join("round");
        context.set_line_cap("round");
        context.begin_path();

        if self.points.len() < 3 {
            let b = self.points.first().unwrap();
            context
                .arc(b.x, b.y, self.pen_size / 2.0, PI * 2.0, 0.0)
                .unwrap();
        } else {
            let first = self.points.first().unwrap();
            let points = self.points.clone();
            context.move_to(first.x, first.y);

            for i in 1..self.points.len() - 2 {
                // let a = self.points[i - 1];
                let a = &points[i];
                let c = &points[i + 1];

                let b: Point = Point::new((a.x + c.x) / 2.0, (a.y + c.y) / 2.0);
                // context.move_to(a.x, a.y);
                context.quadratic_curve_to(a.x, a.y, b.x, b.y);
            }
            let second_last = &points[points.len() - 2];
            let last = self.points.last().unwrap();

            context.quadratic_curve_to(second_last.x, second_last.y, last.x, last.y);
            context.stroke();
        }

        context.fill();
    }

    pub fn draw(&mut self, context: &CanvasRenderingContext2d) {
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
            ToolType::Circle => {
                context.begin_path();
                let last_point = self.points.last().unwrap();
                let first_point = self.points.first().unwrap();
                let radius = ((last_point.x - first_point.x).powi(2)
                    + (last_point.y - first_point.y).powi(2))
                .sqrt();
                console_log!(
                    "first_point: {:?}, last_point: {:?}, radius: {:?}",
                    first_point,
                    last_point,
                    radius
                );
                context
                    .arc(first_point.x, first_point.y, radius, 0.0, PI * 2.0)
                    .unwrap();
            }
            ToolType::Fill => {
                if self.processed {
                    self.draw_quadratic_curve(context)
                } else if self.completed && !self.processed {
                    let canvas = context.canvas().unwrap();
                    let dimensions = get_element_dimensions(&canvas);
                    let mut img_data = context
                        .get_image_data(0.0, 0.0, dimensions.width.into(), dimensions.height.into())
                        .unwrap();

                    let start: SimplePoint = self.points.last().unwrap().clone().into();

                    let res = flood_fill(
                        &mut img_data,
                        dimensions.width,
                        dimensions.height,
                        start.x,
                        start.y,
                    );

                    self.processed = true;
                    self.points = res
                        .iter()
                        .map(|p| Point::new(p.x as f64, p.y as f64))
                        .collect();
                    self.draw_quadratic_curve(context);
                } else {
                    return;
                }
            }
        }

        context.stroke();
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

fn flood_fill(
    canvas: &mut ImageData,
    width: u32,
    height: u32,
    original_x: u32,
    original_y: u32,
    // fill_color: Color,
) -> Vec<SimplePoint> {
    let original_color = get_pixel_color(canvas, original_x, original_y);

    let mut x = original_x;
    let mut y = original_y;

    // First, go up until we find a boundary
    while y > 0 && get_pixel_color(canvas, x, y - 1) == original_color {
        y -= 1;
    }

    // Loop around until we return to the starting point
    let mut path: Vec<SimplePoint> = vec![SimplePoint { x, y }];
    let mut first_iteration = true;
    let mut iteration_count = 0;
    let mut orientation = 1; // 0: ↑, 1: ←, 2: ↓, 3: →

    while (path[path.len() - 1] != path[0]) || first_iteration {
        iteration_count += 1;
        first_iteration = false;

        let (cur_x, cur_y) = path.last().unwrap().to_tuple();
        let mut got_it = false;

        // Determine the current orientation based on the last two points
        if path.len() >= 2 {
            let (prev_x, prev_y) = path[path.len() - 2].to_tuple();
            orientation = match (cur_x as i32 - prev_x as i32, cur_y as i32 - prev_y as i32) {
                (0, -1) => 0, // Up
                (-1, 0) => 1, // Left
                (0, 1) => 2,  // Down
                (1, 0) => 3,  // Right
                _ => orientation,
            };
        }

        // Try moving in 4 directions relative to the current orientation
        for look_at in 0..4 {
            let dir = (orientation + look_at) % 4;
            match dir {
                0 => {
                    // Try moving right
                    if cur_x + 1 < width
                        && get_pixel_color(canvas, cur_x + 1, cur_y) == original_color
                    {
                        x = cur_x + 1;
                        got_it = true;
                    }
                }
                1 => {
                    // Try moving up
                    if cur_y > 0 && get_pixel_color(canvas, cur_x, cur_y - 1) == original_color {
                        y = cur_y - 1;
                        got_it = true;
                    }
                }
                2 => {
                    // Try moving left
                    if cur_x > 0 && get_pixel_color(canvas, cur_x - 1, cur_y) == original_color {
                        x = cur_x - 1;
                        got_it = true;
                    }
                }
                3 => {
                    // Try moving down
                    if cur_y + 1 < height
                        && get_pixel_color(canvas, cur_x, cur_y + 1) == original_color
                    {
                        y = cur_y + 1;
                        got_it = true;
                    }
                }
                _ => {}
            }

            if got_it {
                path.push(SimplePoint { x, y });

                break;
            }
        }
    }
    path
    // Call draw_quadratic_curve (simplified for this example)
    // draw_quadratic_curve(path, canvas, fill_color);
}

fn get_pixel_color(canvas: &ImageData, x: u32, y: u32) -> Color {
    let pixel = get_pixel(canvas, x, y);
    Color {
        r: pixel[0],
        g: pixel[1],
        b: pixel[2],
        a: pixel[3],
    }
}
fn get_pixel(canvas: &ImageData, x: u32, y: u32) -> [u8; 4] {
    let idx = (y * canvas.width() + x) as usize * 4;
    [
        canvas.data()[idx],
        canvas.data()[idx + 1],
        canvas.data()[idx + 2],
        canvas.data()[idx + 3],
    ]
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
