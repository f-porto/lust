pub mod ast;
pub mod expression;
pub mod prefix_expression;
pub mod statement;

use pest_derive::Parser;

#[derive(Debug, Parser)]
#[grammar = "parser/lua.pest"]
pub struct LuaParser;
