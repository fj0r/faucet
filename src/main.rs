use sycamore::prelude::*;
mod libs;
use libs::store::use_web_socket;

#[component]
fn App() -> View {
    let _r = use_web_socket("ws://localhost:3000/channel");

    view! {
        div(class="f v") {
            h1 { "Hello, world!" }
            p { "This is my first Sycamore app" }
        }
    }
}

fn main() {
    sycamore::render(App);
}
