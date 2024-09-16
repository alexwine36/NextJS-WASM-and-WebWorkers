// mod lib;

use wasm_bindgen::JsCast;
use wasm_draw::app::App;
use wasm_draw::init_app;
use wasm_draw::tool::ToolType;
// mod lib;
use leptos::*;
use rand::Rng;
use test_web::Layout;
use web_sys::{window, HtmlCanvasElement};

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    // let app = ;
    // provide_context(App::new(canvas_el));
    // let app = expect_context::<RwSignal<App>>();
    view! {
        <Layout />
        <button
            on:click=move |_| {
                // on stable, this is set_count.set(3);
                set_count.set(3);
            }
        >
            "Click me: "
            // on stable, this is move || count.get();
            // {move || count()}
            {count}
        </button>
    }
}

fn main() {
    console_error_panic_hook::set_once();

    let document = window()
        .and_then(|win| win.document())
        .expect("Could not access the document");
    let body = document.body().expect("Could not access document.body");
    let text_el = document.create_element("p").unwrap();

    text_el.set_text_content("Hello, world from Vanilla Rust!".into());
    body.append_child(text_el.as_ref())
        .expect("Failed to append text");

    let canvas_el: HtmlCanvasElement = document
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    canvas_el
        .set_attribute(
            "style",
            "margin: 50px; width: 50vw; height: 50vh; border: 1px solid black;",
        )
        .unwrap();
    body.append_child(&canvas_el)
        .expect("Failed to append canvas");

    // mount_to_body(|| view! { <App  /> });

    // init_app(canvas_el).expect("Failed to initialize app");
    let mut app = App::new(canvas_el);
    app.set_active_tool(ToolType::Rectangle);
    let state = app.get_dimensions();

    for _ in 0..100 {
        let measurement = app.start_drawing();
        for _ in 0..5 {
            let x = rand::thread_rng().gen_range(0.0..state.width.into());
            let y = rand::thread_rng().gen_range(0.0..state.height.into());

            measurement.borrow_mut().add_point(x, y);
        }
    }

    app.run();
    app.draw();
    app.set_active_tool(ToolType::Fill);
}
