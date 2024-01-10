use std::io::{stdin, BufRead};

use lust::{LuaParser, Rule};
use pest::Parser;

fn main() {
    let mut stdin = stdin().lock();
    let mut line = String::new();
    while let Ok(size) = stdin.read_line(&mut line) {
        if size == 0 {
            break;
        }
        let pairs = LuaParser::parse(Rule::All, &line);
        let Ok(mut pairs) = pairs else {
            println!("{}", pairs.err().unwrap());
            continue;
        };
        for pair in pairs.next().unwrap().into_inner() {
            println!("{:?}", pair);
        }
        line = String::new();
    }
}
