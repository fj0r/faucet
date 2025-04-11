use sycamore::prelude::*;
use super::super::data::Layout;

#[component(inline_props)]
pub fn Layout(layout: ReadSignal<Layout>) -> View {
    let k = move || layout.get_clone().kind;
    view!{
        div(class="ss") {
            (k)
        }
    }
}
