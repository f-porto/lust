use std::io::{stdin, Read};

use lust::parser::{ast::build_ast, LuaParser, Rule};
use pest::Parser;

fn main() {
    let mut stdin = stdin().lock();
    let mut content = String::new();
    stdin.read_to_string(&mut content).unwrap();
    let pairs = LuaParser::parse(Rule::Chunk, &content);
    let Ok(mut pairs) = pairs else {
        println!("{}", pairs.err().unwrap());
        return;
    };
    let program = build_ast(&mut pairs);
    println!("{:?}", program);
}
