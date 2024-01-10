use pest_derive::Parser;

#[derive(Debug, Parser)]
#[grammar = "lua.pest"]
pub struct LuaParser;
