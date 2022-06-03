extern crate lifeline;
use lifeline::parser::parse;
use lifeline::tools::arguments::args;
use lifeline::tools::arguments::Config;
use lifeline::analysis::analyze;
use lifeline::parser::validator::validate;

fn main() {
    let config: Config = args();
    let cfg = parse(&config);
    validate(&cfg, &config);
    analyze(&cfg, &config);
}
