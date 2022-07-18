mod parser;
mod ast;

use std::env;
use std::fs;
use pest::Parser;
use crate::ast::CaravelScriptNode;
use crate::parser::CaravelScriptParser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[0];
    let contents = fs::read_to_string(filename).expect("The given CaravelScript file failed to be read.");
    let prog = CaravelScriptParser::parse(parser::Rule::program, contents);

}
