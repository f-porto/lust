pub mod ast;
mod expression;
mod prefix_expression;
mod statement;

use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;

#[derive(Debug, Parser)]
#[grammar = "lua.pest"]
pub struct LuaParser;

fn print_pair(pair: &Pair<Rule>) {
    println!(
        "{:?} ({}): {:?}",
        pair.as_rule(),
        pair.as_node_tag().unwrap_or("x"),
        pair.as_str()
    );
}

fn h_print_pairs(pairs: Pairs<Rule>, ident: usize) {
    for pair in pairs {
        print!("{}", " ".repeat(ident));
        print_pair(&pair);
        h_print_pairs(pair.into_inner(), ident + 2);
    }
}

fn print_pairs(pairs: Pairs<Rule>) {
    for pair in pairs {
        print_pair(&pair);
        h_print_pairs(pair.into_inner(), 2);
    }
}
