use std::io::Read;
use std::process;
use clap::{Parser};
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

pub fn args() -> Config {
    let arguments:Args = Args::parse();
    Config { source_text: read_source(arguments.src) }}


fn read_source(path: String) -> String{
    let source_file:Result<File,std::io::Error> = File::open(&path);
    match source_file{
        Ok(mut valid_file) => {
            let mut source_file_contents:String = String::new();
            
            match valid_file.read_to_string(&mut source_file_contents) {
                Ok(_) => {
                    return source_file_contents;
                }
                Err(_) => {
                    output::error(format!("Unable to read from {}.", path));
                    process::exit(1);
                }
            }
        }
        Err(_) => {
            output::error(format!("The specified input file {} does not exist.", path));
            process::exit(1);
        }
    }

}