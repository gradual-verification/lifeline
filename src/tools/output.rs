use colored::*;

#[cfg(target_arch = "wasm32")]
use crate::react_bindings::print_frontend;

#[cfg(target_arch = "wasm32")]
fn output(out: String) {
    print_frontend(&out)
}

#[cfg(target_arch = "wasm32")]
fn output_err(out: String) {
    print_frontend(&out)
}

#[cfg(not(target_arch = "wasm32"))]
fn output(out: String) {
    println!("{}", out)
}
#[cfg(not(target_arch = "wasm32"))]
fn output_err(out: String) {
    eprintln!("{}", out)
}

fn fmt_header(symb: String) -> String {
    format!("[{}] — ", symb)
}

pub fn success(msg: String){
    output(format!("{}{}", fmt_header("✓".green().to_string()), msg));
}

pub fn info(msg: String){
    output(format!("{}{}", fmt_header("*".purple().to_string()), msg));
}

pub fn error(msg: String){
    output_err(format!("{}{}", fmt_header("x".red().to_string()), msg));
}

pub fn warning(msg: String){
    output_err(format!("{}{}", fmt_header("x".yellow().to_string()), msg));
}

pub fn debug(msg: String){
    output_err(format!("{}{}", fmt_header("D".cyan().to_string()), msg));
}