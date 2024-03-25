#![allow(dead_code)]

use std::collections::HashMap;

use crate::parser::statement::{Block, Statement};

// Not necessary yet, I believe

#[derive(Debug)]
pub struct SymbolTable {
    table: HashMap<String, Item>,
    parent: Option<Box<SymbolTable>>,
}

#[derive(Debug)]
pub enum Item {
    Variable,
    LocalVariable,
    Function(SymbolTable),
    LocalFunction(SymbolTable),
}

impl SymbolTable {
    pub fn new(block: &Block) -> Self {
        let mut symbol_table = SymbolTable {
            table: HashMap::new(),
            parent: None,
        };
        symbol_table.build_symbol_table(block);
        symbol_table
    }

    fn build_symbol_table(&mut self, block: &Block) {
        for statement in block.statements.iter() {
            match statement {
                Statement::LocalVariables { .. } => self.deal_with_local_variables(statement),
                _ => todo!("{:?}", statement),
            }
        }
    }

    fn deal_with_local_variables(&mut self, statement: &Statement) {
        let Statement::LocalVariables {
            variables,
            expr_list,
        } = statement
        else {
            unreachable!("Expected local variables, got {:?}", statement);
        };
        if let Some(_) = expr_list {}
        for _ in variables {}
    }
}
