use std::{error::Error, fs};

use lust::parser::{LuaParser, Rule};
use pest::{iterators::Pairs, Parser};
use pretty_assertions::*;

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

#[test]
fn markov() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("./lua/markov_chain_algorithm.lua").unwrap();
    let pairs = LuaParser::parse(Rule::Chunk, &content);
    let Ok(pairs) = pairs else {
        let err = pairs.err().unwrap();
        println!("{}", err);
        return Err(err.into());
    };
    print_pairs(pairs, 0);
    Ok(())
}
