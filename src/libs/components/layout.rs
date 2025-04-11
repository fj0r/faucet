use sycamore::prelude::*;
use super::super::data::Layout;

#[component(inline_props)]
pub fn Layout(layout: ReadSignal<Layout>) -> View {
    create_effect(move || {
        console_log!("{layout:?}");
    });
    view!{
        ""
    }
}
