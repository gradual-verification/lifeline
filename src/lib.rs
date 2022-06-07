pub mod tools;
use crate::tools::arguments::*;

pub mod parser;
use crate::parser::*;

pub mod analyzer;
use crate::analyzer::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn format(source: &str) {

}
#[wasm_bindgen]
pub fn run(source: &str) {

}

#[wasm_bindgen]
extern "C" {
    pub fn log(output: &str);
}