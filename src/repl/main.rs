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
    ASTBuilder::new().build_ast(pairs);

    // while let Ok(size) = stdin.read_line(&mut line) {
    //     if size == 0 {
    //         break;
    //     }
    //     let pairs = LuaParser::parse(Rule::All, &line);
    //     let Ok(mut pairs) = pairs else {
    //         println!("{}", pairs.err().unwrap());
    //         continue;
    //     };
    //     for pair in pairs.next().unwrap().into_inner() {
    //         println!("{:?}", pair);
    //     }
    //     line = String::new();
    // }
}
