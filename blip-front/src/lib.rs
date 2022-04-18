extern crate blip_domain;
extern crate wasm_bindgen;

use std::collections::HashMap;

use blip_domain::domain::Item;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct BlipItem {
    pub author: String,
    pub body: String,
    pub time: u64,
    pub mask: String,
}

#[wasm_bindgen]
pub fn compute(item_json: JsValue) -> JsValue {
    use web_sys::console;
    console::log_1(&item_json);
    let item: BlipItem = item_json.into_serde().unwrap();
    let item = Item {
        author: item.author,
        body: item.body,
        time: item.time,
        mask: item.mask,
        attributes: HashMap::default(),
    };
    let code = item.code();
    JsValue::from_serde(&(code.clone(), item.masked(code).unwrap_or_default())).unwrap()
}
