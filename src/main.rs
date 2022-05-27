extern crate lifeline;
use lifeline::parser::parse;
use lifeline::tools::arguments::args;
use lifeline::tools::arguments::Config;
use lifeline::analysis::analyze;

fn main() {
    let config: Config = args();
    let cfg = parse(&config);
    analyze(cfg, config);
}


