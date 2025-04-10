use std::str;

use super::ws::use_web_socket;
use js_sys::wasm_bindgen::JsError;
use sycamore::prelude::*;
use sycamore_futures::spawn_local_scoped;
use serde_json::value::{to_value, Value};
use serde::{Serialize, Deserialize};
use anyhow::Result;

#[derive(Serialize, Deserialize)]
pub struct Layout {
    pub kind: String,
    pub data: String,
    pub item: Option<Box<Layout>>,
    pub children: Option<Box<Layout>>
}

#[derive(Serialize, Deserialize)]
pub struct Empty;

#[derive(Serialize, Deserialize)]
#[serde(tag = "action")]
pub enum Message {
    Layout,
}


#[derive(Clone, Copy)]
pub struct Store {
    pub layout: ReadSignal<String>,
}

pub fn use_store(url: &str) -> Result<Store, JsError> {
    let ws = use_web_socket(url)?;
    let layout = move || {
        // let x = d.message();
        // serde_json::from_str::<Message>(&x).unwrap_or_else(|_| Empty{})
        ws.message()
    };
    //let x = std::str::from_utf8(d.message_bytes().try_into())?;

    Ok(Store {
        layout: ws.message(),
    })
}
