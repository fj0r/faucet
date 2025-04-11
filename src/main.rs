use sycamore::prelude::*;
mod libs;
use libs::{store::use_store, ws::use_web_socket};

#[component]
fn App() -> View {
    let url = "ws://localhost:3000/channel";
    let r = use_store(url).expect("connecting failed");


    let signal = create_signal(1);

    create_effect(move || {
        let s = signal.get();
        console_log!("start: {s}");
    });

    let l = r.layout.get_clone();
    view! {
        div(class="f v") {
            h1 { "Hello, world!" }
            p { "This is my first Sycamore app" }
            p { (signal) }
            button(on:click=move |_| signal.set(signal.get() + 1)) { "+1" }
            p { "--" }
            p { (l.kind) }
        }
    }
}

fn main() {
    sycamore::render(App);
}
