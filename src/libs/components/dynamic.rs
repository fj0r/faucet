use super::super::data::Layout;
use super::container::Container;
use super::widgets::*;
use sycamore::prelude::*;

/*
#[component(inline_props)]
pub fn Dynamic(
    #[prop(attributes(html, button))]
    attributes: Attributes,
    children: Children,
    type: &str
) -> View {
    view!{
        button(...attributes)
    }
}
*/

#[component(inline_props)]
pub fn Dynamic(attributes: Attributes, children: Children, kind: ReadSignal<String>) -> View {
    let c =  {
        let k = kind.get_clone();
        match k.as_str() {
            "Container" => view!( div { "Container" } ),
            "Input" => view! ( div { "input" } ),
            "Text" => view! ( div { "Text" } ),
            "Card" => view! ( div { "Card" } ),
            _ => view! { div { (format!("{} unimplemented!", &kind)) } },
        }
    };
    view! {
        p { (c) }
    }
}
