use crate::log;
use crate::settings;
use crate::state::get_element_dimensions;
use crate::state::Dimensions;
use crate::state::State;
use crate::tool::ToolType;

use std::cell::RefCell;
use std::rc::Rc;
use utilities::console_log;
use wasm_bindgen::prelude::*;
use web_sys::ResizeObserver;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};

use crate::tool::Measurement;

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct App {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    state: Rc<RefCell<State>>,
    measurements: Rc<RefCell<Vec<Rc<RefCell<Measurement>>>>>,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> App {
        let mut settings = settings::Settings::new();

        settings.add_color("something", "#aa00bb");
        let state: Rc<RefCell<State>> = Rc::new(RefCell::new(State::new(&canvas, settings)));
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        canvas.set_width(state.borrow().get_width());
        canvas.set_height(state.borrow().get_height());
        App {
            canvas,
            context,
            state,
            measurements: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn initialize_canvas(&self) {
        // console_log!("initializing canvas");
        let dimensions = get_element_dimensions(&self.canvas);
        self.canvas.set_width(dimensions.width);
        self.canvas.set_height(dimensions.height);
        self.state.borrow_mut().set_dimensions(dimensions);
    }

    pub fn get_dimensions(&self) -> Dimensions {
        self.state.borrow().get_dimensions()
    }

    pub fn get_state(&self) -> State {
        self.state.borrow().clone()
    }

    pub fn get_active_tool(&self) -> ToolType {
        self.state.borrow().get_tool()
    }

    pub fn set_active_tool(&self, tool: ToolType) {
        self.state.borrow_mut().set_tool(tool);
    }

    pub fn get_colors(&self) -> Vec<String> {
        self.state
            .borrow()
            .get_settings()
            .colors
            .iter()
            .map(|c| c.hex.clone())
            .collect()
    }

    pub fn get_active_color(&self) -> String {
        self.state.borrow().get_color()
    }
    pub fn set_active_color(&self, color: String) {
        self.state.borrow_mut().update_color(color);
    }

    pub fn get_pen_sizes(&self) -> Vec<f64> {
        self.state.borrow().get_settings().pen_sizes.clone()
    }

    pub fn get_active_pen_size(&self) -> f64 {
        self.state.borrow().get_pen_size()
    }

    pub fn set_active_pen_size(&self, pen_size: f64) {
        self.state.borrow_mut().update_pen_size(pen_size);
    }

    pub fn draw(&self) {
        self.context.clear_rect(
            0.0,
            0.0,
            self.canvas.width() as f64,
            self.canvas.height() as f64,
        );

        for measurement in self.measurements.borrow().iter() {
            measurement.borrow_mut().draw(&self.context);
        }
    }

    pub fn run(&mut self) {
        self.initialize_canvas();
        let measurement_ref: Rc<RefCell<Option<Rc<RefCell<Measurement>>>>> =
            Rc::new(RefCell::new(None));

        {
            let self_copy = self.clone();
            let handle_window_resize = Closure::wrap(Box::new(move || {
                self_copy.initialize_canvas();
                self_copy.draw();
            }) as Box<dyn FnMut()>);

            let parent = self.canvas.clone().parent_element().unwrap();

            let resize_observer =
                ResizeObserver::new(handle_window_resize.as_ref().unchecked_ref())
                    .expect("Could not create ResizeObserver");

            resize_observer.observe(&parent);

            handle_window_resize.forget();
        }

        {
            let state_copy = self.state.clone();
            let mut self_copy = self.clone();
            let measurement = measurement_ref.clone();
            let handle_mouse_down = Closure::wrap(Box::new(move |event: MouseEvent| {
                state_copy.borrow_mut().start_drawing();
                // measurement = Some(self_copy.start_drawing());
                *measurement.borrow_mut() = Some(self_copy.start_drawing());
                let (new_x, new_y) = state_copy.borrow().get_mouse_position(event);

                measurement
                    .borrow_mut()
                    .as_ref()
                    .unwrap()
                    .borrow_mut()
                    .add_point(new_x, new_y);
            }) as Box<dyn FnMut(_)>);

            self.canvas
                .add_event_listener_with_callback(
                    "mousedown",
                    handle_mouse_down.as_ref().unchecked_ref(),
                )
                .unwrap();

            handle_mouse_down.forget();
        }
        {
            let state_copy = self.state.clone();
            let self_copy = self.clone();
            let measurement = measurement_ref.clone();
            let handle_mouse_move = Closure::wrap(Box::new(move |event: MouseEvent| {
                let m = measurement.borrow().clone();

                if let Some(measure) = m {
                    // console_log!("mouse move, {:?}", m);
                    if state_copy.borrow().is_drawing() {
                        let (new_x, new_y) = state_copy.borrow().get_mouse_position(event);
                        // *measurement.borrow_mut().add_point(new_x, new_y);
                        measure.borrow_mut().add_point(new_x, new_y);
                        self_copy.draw();
                    }
                }
            }) as Box<dyn FnMut(_)>);

            self.canvas
                .add_event_listener_with_callback(
                    "mousemove",
                    handle_mouse_move.as_ref().unchecked_ref(),
                )
                .unwrap();

            handle_mouse_move.forget();
        }
        {
            let state_copy = self.state.clone();
            let self_copy = self.clone();
            let measurement = measurement_ref.clone();
            let handle_mouse_up = Closure::wrap(Box::new(move |event: MouseEvent| {
                let (new_x, new_y) = state_copy.borrow().get_mouse_position(event);
                state_copy.borrow_mut().stop_drawing();

                let m = measurement.borrow().clone();
                if let Some(measure) = m {
                    measure.borrow_mut().add_point(new_x, new_y);
                    measure.borrow_mut().finish()
                }
                self_copy.draw();

                *measurement.borrow_mut() = None;
            }) as Box<dyn FnMut(_)>);

            self.canvas
                .add_event_listener_with_callback(
                    "mouseup",
                    handle_mouse_up.as_ref().unchecked_ref(),
                )
                .unwrap();

            handle_mouse_up.forget();
        }
    }
}

impl App {
    pub fn start_drawing(&mut self) -> Rc<RefCell<Measurement>> {
        self.state.borrow_mut().start_drawing();
        let color = self.state.borrow().get_color();
        let pen_size = self.state.borrow().get_pen_size();
        let tool = self.state.borrow().get_tool();
        let measurement = Rc::new(RefCell::new(Measurement::new(&color, pen_size, tool)));

        self.measurements.borrow_mut().push(measurement.clone());

        measurement
    }
}
