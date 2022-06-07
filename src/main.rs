pub mod react_bindings;

pub mod tools;
use crate::tools::arguments::*;

pub mod parser;
use crate::parser::*;

pub mod analyzer;
use crate::analyzer::*;

fn main() {
    let config: Config = args();
    let cfg = parse(&config);
    analyze(cfg, config);
}

