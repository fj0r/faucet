use sycamore::prelude::*;
use super::super::data::Layout;
use super::Dynamic;

#[component(inline_props)]
pub fn Layout(layout: ReadSignal<Layout>) -> View {
    let k = create_memo(move || layout.get_clone().kind);
    view!{
        Dynamic(kind=k) { "" }
    }
}
