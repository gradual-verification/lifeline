use crate::tools::arguments::Config;

pub mod cfg;
use crate::cfg::*;


pub(crate) fn parse(config: & Config) -> CFG {
    CFG {procedures: vec![], structs: vec![]}
}
