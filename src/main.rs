use sycamore::prelude::*;
mod libs;
use libs::{store::use_store, ws::use_web_socket};
use libs::components::*;

#[component]
fn App() -> View {
    let url = "ws://localhost:3000/channel";
    let r = use_store(url).expect("connecting failed");


    let count = create_signal(1);

    create_effect(move || {
        let s = count.get();
        console_log!("start: {s}");
    });

    view! {
        div(class="f v") {
            Layout(layout=r.layout) {}
            h1 { "Hello, world!" }
            p { "This is my first Sycamore app" }
            (count)
            button(on:click=move |_| count.set(count.get() + 1)) { "+1" }
            p { "--" }
        }
    }
}

fn main() {
    sycamore::render(App);
}
