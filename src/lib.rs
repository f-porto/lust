mod expression;
mod prefix_expression;
mod statement;
use pest_derive::Parser;

#[derive(Debug, Parser)]
#[grammar = "lua.pest"]
pub struct LuaParser;
