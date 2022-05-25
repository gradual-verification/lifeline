use std::io::Read;
use std::process;
use clap::{Parser, Arg};
use std::fs::File;

use crate::tools::output;

//A minimal language and analysis framework for prototyping lifetime inference
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    src: String
}

pub struct Config {
    source_text: String,
}

pub(crate) fn args() -> Config {
    let arguments:Args = Args::parse();
    Config { source_text: read_source(arguments.src) }}


fn read_source(path: String) -> String{
    let mut source_file:Result<File,std::io::Error> = File::open(&path);
    match source_file{
        Ok(mut valid_file) => {
            let mut source_file_contents:String = String::new();
            valid_file.read_to_string(&mut source_file_contents);
            source_file_contents
        }
        Err(_) => {
            output::error(format!("The specified input file {} does not exist.", path));
            process::exit(1);
        }
    }

}