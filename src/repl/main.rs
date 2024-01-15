use std::io::{stdin, Read};

use lust::{ast::ASTBuilder, LuaParser, Rule};
use pest::Parser;

fn main() {
    let mut stdin = stdin().lock();
    let mut content = String::new();
    stdin.read_to_string(&mut content).unwrap();
    let pairs = LuaParser::parse(Rule::Chunk, &content);
    let Ok(pairs) = pairs else {
        println!("{}", pairs.err().unwrap());
        return;
    };
    let program =  ASTBuilder::new().build_ast(pairs);
    println!("{:?}", program);
}
