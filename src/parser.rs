use pest_derive::Parser;
use derive;
#[derive(Parser)]
#[grammar = "main.pest"]
pub struct CaravelScriptParser;
