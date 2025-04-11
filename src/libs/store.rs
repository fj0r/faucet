use std::str;

use super::ws::use_web_socket;
use js_sys::wasm_bindgen::JsError;
use sycamore::prelude::*;
use sycamore_futures::spawn_local_scoped;
use serde_json::value::{to_value, Value};
use serde::{Serialize, Deserialize};
use anyhow::Result;
use super::data::{Layout, Empty};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(tag = "action")]
pub enum Message {
    #[warn(non_camel_case_types)]
    layout(Layout),

    #[warn(non_camel_case_types)]
    #[default]
    empty,
}


#[derive(Clone, Copy)]
pub struct Store {
    pub layout: ReadSignal<Layout>,
}

pub fn use_store(url: &str) -> Result<Store, JsError> {
    let ws = use_web_socket(url)?;
    let x = ws.message();
    let act = serde_json::from_str::<Message>(&x.get_clone())
        .unwrap_or_else(|_| Message::empty{});
    let layout = create_signal::<Layout>(Layout::default());
    match act {
        Message::layout(x) => layout.set(x),
        Message::empty => ()
    };
    Ok(Store{
        layout: *layout
    })
}
