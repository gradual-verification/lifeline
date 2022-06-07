use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn pretty_print(source: &str) -> String{
    format!("")
}

#[wasm_bindgen]
pub fn analyze(source: &str) {

}

extern "C" {
    pub fn alert(s: &str);
}

pub fn print_frontend(input: &str) {
    unsafe {alert(input);}
}