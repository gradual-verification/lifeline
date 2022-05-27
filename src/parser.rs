pub mod cfg;
use cfg::*;
use crate::tools::arguments::Config;

pub fn parse(config: & Config) -> CFG {
    CFG {procedures: vec![], structs: vec![]}
}