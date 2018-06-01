#![feature(proc_macro, wasm_import_module, wasm_custom_section)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn hello_world() {
    alert("Hello World!");
}