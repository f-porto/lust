use std::io::{stdin, Read};

use lust::{LuaParser, Rule, ExpressionParser};
use pest::Parser;

fn main() {
    let mut stdin = stdin().lock();
    let mut content = String::new();
    stdin.read_to_string(&mut content).unwrap();
    let pairs = LuaParser::parse(Rule::Expression, &content);
    let Ok(mut pairs) = pairs else {
        println!("{}", pairs.err().unwrap());
        return;
    };
    let expr_parser = ExpressionParser::new();
    let expr_tree = ExpressionParser::parse_expr(&expr_parser, pairs.next().unwrap().into_inner());
    println!("{:?}", expr_tree);

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
