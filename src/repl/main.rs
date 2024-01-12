use std::io::{stdin, Read};

use lust::{LuaParser, Rule};
use pest::{iterators::Pairs, Parser};

fn print_pairs(pairs: Pairs<Rule>, ident: usize) {
    for pair in pairs {
        println!(
            "{}{:?}: {:?}",
            " ".repeat(ident),
            pair.as_rule(),
            pair.as_str()
        );
        print_pairs(pair.into_inner(), ident + 2);
    }
}

fn main() {
    let mut stdin = stdin().lock();
    let mut content = String::new();
    stdin.read_to_string(&mut content).unwrap();
    let pairs = LuaParser::parse(Rule::Chunk, &content);
    let Ok(pairs) = pairs else {
        println!("{}", pairs.err().unwrap());
        return;
    };
    print_pairs(pairs, 0);

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
