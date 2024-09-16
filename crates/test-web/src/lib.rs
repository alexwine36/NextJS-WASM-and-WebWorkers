use leptos::*;
use wasm_draw::app::App;
#[component]
pub fn Layout() -> impl IntoView {
    // let app = use_context::<ReadSignal<App>>().expect("Where you at");

    // app.g
    // app().run();
    view! {
        <div>
            <h1>"Hello, world!"</h1>

        </div>
    }
}
