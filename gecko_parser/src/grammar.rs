use pest_consume::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct GeckoParser;