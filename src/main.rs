use sycamore::prelude::*;

#[component]
fn App() -> View {
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
