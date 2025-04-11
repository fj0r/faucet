use std::str;

use super::{ws::use_web_socket};
use js_sys::wasm_bindgen::JsError;
use sycamore::prelude::*;
use sycamore_futures::spawn_local_scoped;
use serde_json::value::{to_value, Value};
use serde::{Serialize, Deserialize};
use anyhow::Result;
use super::data::*;


#[derive(Clone, Copy)]
pub struct Store {
    pub layout: ReadSignal<Layout>,
}

pub fn use_store(url: &str) -> Result<Store, JsError> {
    let ws = use_web_socket(url)?;
    let x = ws.message();

    let layout = create_signal::<Layout>(Layout::default());

    create_memo(move|| {
        let act = serde_json::from_str::<Message>(&x.get_clone())
            .unwrap_or_else(|_| Message::default());
        match act {
            Message{content: Content::layout(x), ..} => layout.set(x),
            Message{user: _, content: Content::empty} => (),
        };
    });

    Ok(Store{
        layout: *layout
    })
}

