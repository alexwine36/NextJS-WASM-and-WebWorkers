extern crate wasm_bindgen;

use crate::canvas;
use crate::log;
use crate::state::State;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use utilities::console_log;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};

#[derive(Eq, Hash, PartialEq)]
enum ToolType {
    Line,
    // Circle,
    Rectangle,
}

pub struct ToolBelt {
    // pub tools: HashMap<ToolType, Box<dyn Tool>>, //Vec<Box<dyn Tool>>,
    // pub active_tool: Tool,
    tool_type: ToolType,
    canvas: HtmlCanvasElement,
    state: Rc<RefCell<State>>,
    context: CanvasRenderingContext2d,
}

impl ToolBelt {
    pub fn new(canvas: &HtmlCanvasElement, state: &Rc<RefCell<State>>) -> Self {
        // let line_tool = Box::new(Line::new());
        // let mut available_tools: HashMap<ToolType, Box<dyn Tool>> = HashMap::new();
        // available_tools.insert(ToolType::Line, line_tool);
        Self {
            tool_type: ToolType::Rectangle,
            // tools: available_tools,
            // tools: ,
            // active_tool: Some(line_tool),
            canvas: canvas.clone(),
            state: state.clone(),
            context: canvas
                .get_context("2d")
                .expect("Could not get context")
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap(),
        }
    }

    pub fn set_active_tool(&mut self, tool_type: ToolType) {
        self.tool_type = tool_type;
    }

    pub fn get_active_tool(&self) -> Box<dyn Tool> {
        let res: Box<dyn Tool> = match self.tool_type {
            ToolType::Line => Box::new(Line::new()),
            ToolType::Rectangle => Box::new(Rectangle::new()),
        };
        res
    }
    pub fn init(&self) {
        let mut tool = self.get_active_tool();

        tool.init(&self.context, &self.canvas, &self.state);
    }
}

pub trait Tool {
    fn on_mouse_down(
        &mut self,
        context: &CanvasRenderingContext2d,
        canvas: &HtmlCanvasElement,
        state: &Rc<RefCell<State>>,
    ) -> Closure<dyn FnMut(MouseEvent)>;
    fn on_mouse_up(
        &mut self,
        context: &CanvasRenderingContext2d,
        canvas: &HtmlCanvasElement,
        state: &Rc<RefCell<State>>,
    ) -> Closure<dyn FnMut(MouseEvent)>;
    fn on_mouse_move(
        &mut self,
        context: &CanvasRenderingContext2d,
        canvas: &HtmlCanvasElement,
        state: &Rc<RefCell<State>>,
    ) -> Closure<dyn FnMut(MouseEvent)>;

    fn init(
        &mut self,
        context: &CanvasRenderingContext2d,
        canvas: &HtmlCanvasElement,
        state: &Rc<RefCell<State>>,
    ) {
        let handle_mouse_down = self.on_mouse_down(context, canvas, state);
        let handle_mouse_up = self.on_mouse_up(context, canvas, state);
        let handle_mouse_move = self.on_mouse_move(context, canvas, state);

        let _ = canvas.add_event_listener_with_callback(
            "mousedown",
            handle_mouse_down.as_ref().unchecked_ref(),
        );

        let _ = canvas
            .add_event_listener_with_callback("mouseup", handle_mouse_up.as_ref().unchecked_ref());

        let _ = canvas.add_event_listener_with_callback(
            "mousemove",
            handle_mouse_move.as_ref().unchecked_ref(),
        );

        handle_mouse_down.forget();
        handle_mouse_up.forget();
        handle_mouse_move.forget();
    }
}

#[derive(Debug, Clone)]
pub struct Line {
    prev_point: Rc<RefCell<Option<(f64, f64)>>>,
}

impl Line {
    pub fn new() -> Self {
        Self {
            prev_point: Rc::new(RefCell::new(None)),
        }
    }
}

impl Tool for Line {
    fn on_mouse_down(
        &mut self,
        context: &CanvasRenderingContext2d,
        canvas: &HtmlCanvasElement,
        state: &Rc<RefCell<State>>,
    ) -> Closure<dyn FnMut(MouseEvent)> {
        let context_copy = context.clone();
        let state_copy = state.clone();
        let canvas_copy = canvas.clone();

        let prev_point = self.prev_point.clone();
        Closure::wrap(Box::new(move |event: MouseEvent| {
            state_copy.borrow_mut().start_drawing();
            state_copy
                .borrow_mut()
                .add_undo_state(canvas_copy.to_data_url().unwrap());
            let (new_x, new_y) = state_copy.borrow().get_mouse_position(event);
            prev_point.borrow_mut().replace((new_x, new_y));

            context_copy.begin_path();
            context_copy.set_stroke_style(&JsValue::from(state_copy.borrow().get_color()));
            context_copy.set_line_width(state_copy.borrow().get_pen_size());
            context_copy.move_to(new_x, new_y);
        }) as Box<dyn FnMut(_)>)
    }

    fn on_mouse_up(
        &mut self,
        context: &CanvasRenderingContext2d,
        canvas: &HtmlCanvasElement,
        state: &Rc<RefCell<State>>,
    ) -> Closure<dyn FnMut(MouseEvent)> {
        let context_copy = context.clone();
        let state_copy = state.clone();
        let canvas_copy = canvas.clone();

        Closure::wrap(Box::new(move |event: MouseEvent| {
            state_copy.borrow_mut().stop_drawing();
            let (new_x, new_y) = state_copy.borrow().get_mouse_position(event);
            // context_copy.clear_rect(
            //     0.0,
            //     0.0,
            //     state_copy.borrow().get_width().into(),
            //     state_copy.borrow().get_height().into(),
            // );
            // context_copy.fill_rect(new_x, new_y, 1.0, 1.0);
            context_copy.line_to(new_x, new_y);
            context_copy.stroke();
        }) as Box<dyn FnMut(_)>)
    }

    fn on_mouse_move(
        &mut self,
        context: &CanvasRenderingContext2d,
        canvas: &HtmlCanvasElement,
        state: &Rc<RefCell<State>>,
    ) -> Closure<dyn FnMut(MouseEvent)> {
        let context_copy = context.clone();
        let state_copy = state.clone();
        let canvas_copy = canvas.clone();

        // let self_copy_head = RefCell::new(self.clone());

        let prev_point_head = self.prev_point.clone();

        Closure::wrap(Box::new(move |event: MouseEvent| {
            if state_copy.borrow().is_drawing() {
                let (new_x, new_y) = state_copy.borrow().get_mouse_position(event);
                context_copy.clear_rect(
                    0.0,
                    0.0,
                    state_copy.borrow().get_width().into(),
                    state_copy.borrow().get_height().into(),
                );
                // let self_copy = self_copy_head.borrow_mut();
                // let prev_point = self_copy_head.borrow().prev_point.clone();
                let prev_point = prev_point_head.borrow().clone();

                if let Some((prev_x, prev_y)) = prev_point {
                    context_copy.begin_path();
                    // context_copy.set_stroke_style(&JsValue::from(state_copy.borrow().get_color()));
                    // context_copy.set_line_width(state_copy.borrow().get_pen_size());
                    context_copy.move_to(prev_x, prev_y);
                    context_copy.line_to(new_x, new_y);
                    context_copy.stroke();
                }

                // context_copy.line_to(new_x, new_y);
                // context_copy.stroke();
            }
        }) as Box<dyn FnMut(_)>)
    }
}

#[derive(Debug, Clone)]
pub struct Rectangle {
    prev_point: Rc<RefCell<Option<(f64, f64)>>>,
}

impl Rectangle {
    pub fn new() -> Self {
        Self {
            prev_point: Rc::new(RefCell::new(None)),
        }
    }
}

impl Tool for Rectangle {
    fn on_mouse_down(
        &mut self,
        context: &CanvasRenderingContext2d,
        canvas: &HtmlCanvasElement,
        state: &Rc<RefCell<State>>,
    ) -> Closure<dyn FnMut(MouseEvent)> {
        let context_copy = context.clone();
        let state_copy = state.clone();
        let canvas_copy = canvas.clone();

        let prev_point = self.prev_point.clone();
        Closure::wrap(Box::new(move |event: MouseEvent| {
            state_copy.borrow_mut().start_drawing();
            state_copy
                .borrow_mut()
                .add_undo_state(canvas_copy.to_data_url().unwrap());
            let (new_x, new_y) = state_copy.borrow().get_mouse_position(event);
            prev_point.borrow_mut().replace((new_x, new_y));

            context_copy.begin_path();
            context_copy.set_stroke_style(&JsValue::from(state_copy.borrow().get_color()));
            context_copy.set_line_width(state_copy.borrow().get_pen_size());
            context_copy.move_to(new_x, new_y);
        }) as Box<dyn FnMut(_)>)
    }

    fn on_mouse_up(
        &mut self,
        context: &CanvasRenderingContext2d,
        canvas: &HtmlCanvasElement,
        state: &Rc<RefCell<State>>,
    ) -> Closure<dyn FnMut(MouseEvent)> {
        let context_copy = context.clone();
        let state_copy = state.clone();
        let canvas_copy = canvas.clone();

        Closure::wrap(Box::new(move |event: MouseEvent| {
            state_copy.borrow_mut().stop_drawing();
            let (new_x, new_y) = state_copy.borrow().get_mouse_position(event);
            // context_copy.clear_rect(
            //     0.0,
            //     0.0,
            //     state_copy.borrow().get_width().into(),
            //     state_copy.borrow().get_height().into(),
            // );
            // context_copy.fill_rect(new_x, new_y, 1.0, 1.0);
            context_copy.line_to(new_x, new_y);
            context_copy.stroke();
        }) as Box<dyn FnMut(_)>)
    }

    fn on_mouse_move(
        &mut self,
        context: &CanvasRenderingContext2d,
        canvas: &HtmlCanvasElement,
        state: &Rc<RefCell<State>>,
    ) -> Closure<dyn FnMut(MouseEvent)> {
        let context_copy = context.clone();
        let state_copy = state.clone();
        let canvas_copy = canvas.clone();

        // let self_copy_head = RefCell::new(self.clone());

        let prev_point_head = self.prev_point.clone();

        Closure::wrap(Box::new(move |event: MouseEvent| {
            if state_copy.borrow().is_drawing() {
                let (new_x, new_y) = state_copy.borrow().get_mouse_position(event);
                context_copy.clear_rect(
                    0.0,
                    0.0,
                    state_copy.borrow().get_width().into(),
                    state_copy.borrow().get_height().into(),
                );
                // let self_copy = self_copy_head.borrow_mut();
                // let prev_point = self_copy_head.borrow().prev_point.clone();
                let prev_point = prev_point_head.borrow().clone();

                if let Some((prev_x, prev_y)) = prev_point {
                    // context_copy.begin_path();
                    // // context_copy.set_stroke_style(&JsValue::from(state_copy.borrow().get_color()));
                    // // context_copy.set_line_width(state_copy.borrow().get_pen_size());
                    // context_copy.move_to(prev_x, prev_y);
                    // context_copy.line_to(new_x, new_y);
                    // context_copy.stroke();

                    context_copy.stroke_rect(prev_x, prev_y, new_x - prev_x, new_y - prev_y);
                }

                // context_copy.line_to(new_x, new_y);
                // context_copy.stroke();
            }
        }) as Box<dyn FnMut(_)>)
    }
}
