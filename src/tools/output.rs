use colored::*;

fn fmt_header(symb: String) -> String {
    format!("[{}] — ", symb)
}

pub fn success(msg: String){
    println!("{}{}", fmt_header("✓".green().to_string()), msg);
}

pub fn info(msg: String){
    println!("{}{}", fmt_header("*".purple().to_string()), msg);
}

pub fn error(msg: String){
    eprintln!("{}{}", fmt_header("x".red().to_string()), msg);
}

pub fn warning(msg: String){
    eprintln!("{}{}", fmt_header("x".yellow().to_string()), msg);
}

pub fn debug(msg: String){
    println!("{}{}", fmt_header("D".cyan().to_string()), msg)
}