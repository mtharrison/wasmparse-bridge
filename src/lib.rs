#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;
extern crate wasmparse_core;

extern crate serde;
extern crate serde_json;

use std::io::Cursor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn desc(data: &[u8]) -> String {
    let module = wasmparse_core::parse(Cursor::new(data));
    return format!("{:#?}", module);
}

#[wasm_bindgen]
pub fn json(data: &[u8]) -> String {
    let module = wasmparse_core::parse(Cursor::new(data)).unwrap();
    return serde_json::to_string(&module).unwrap();
}
