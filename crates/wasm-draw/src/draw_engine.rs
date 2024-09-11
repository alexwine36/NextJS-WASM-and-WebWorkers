use std::collections::VecDeque;

enum ShapeType {
    Rectangle,
    // Circle,
    Line,
}

struct Point {
    x: f64,
    y: f64,
}

struct Shape {
    points: Vec<Point>,
    width: f64,
    height: f64,
    color: String,
}

pub struct DrawEngine {
    queue: VecDeque<Shape>,
}

impl DrawEngine {
    pub fn new() -> Self {
        DrawEngine {
            queue: VecDeque::new(),
        }
    }

    pub fn add_shape(&mut self, shape: Shape) {
        self.queue.push_back(shape);
    }

    pub fn undo(&mut self) -> Option<Shape> {
        self.queue.pop_back()
    }

    pub fn redo(&mut self) -> Option<Shape> {
        self.queue.pop_front()
    }
}
