use std::collections::HashMap;

use crate::parser::{
    expression::Expression,
    statement::{Block, Parameters, Statement, Variable},
};

#[derive(Debug)]
pub struct Interpreter<'a> {
    pub program: &'a Block,
    pub scopes: Vec<Scope>,
}

#[derive(Debug)]
pub struct Scope {
    table: HashMap<String, Value>,
}

impl Scope {
    fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    fn insert(&mut self, name: String, value: Value) {
        self.table.insert(name, value);
    }

    fn get(&mut self, name: String) -> &Value {
        self.table.get(&name).unwrap_or(&Value::Nil)
    }
}

#[derive(Debug)]
struct Table {
    table: HashMap<Value, Value>,
}

#[derive(Debug)]
enum Value {
    Nil,
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Table(Table),
    Lambda { parameters: Parameters, body: Block },
}

impl<'a> Interpreter<'a> {
    pub fn new(program: &'a Block) -> Self {
        Self {
            program,
            scopes: vec![],
        }
    }

    pub fn interpret(&mut self) {
        self.evaluate_block(self.program);
    }

    fn evaluate_block(&mut self, block: &Block) {
        self.scopes.push(Scope::new());
        for statement in block.statements.iter() {
            match statement {
                Statement::LocalVariables { .. } => self.evaluate_local_variables(statement),
                Statement::Assignment { .. } => self.evaluate_global_variables(statement),
                Statement::Do(_) => self.evaluate_do(statement),
                Statement::Label(_) => todo!("How to deal with labels"),
                Statement::Goto(_) => todo!("How to deal with goto's"),
                Statement::Empty => {}
                Statement::Break => todo!("Break outside a loop"),
                _ => unreachable!("Expected statement, found {:?}", statement),
            }
        }
        println!("Scope after: {:?}", self.scopes.last().unwrap());
        self.scopes.pop();
    }

    fn evaluate_do(&mut self, statement: &Statement) {
        let Statement::Do(block) = statement else {
            unreachable!("Expected do statement, found {:?}", statement);
        };
        self.evaluate_block(block);
    }

    fn evaluate_global_variables(&mut self, statement: &Statement) {
        let Statement::Assignment {
            variable_list,
            expr_list,
        } = statement
        else {
            unreachable!("Expected assignment, found {:?}", statement);
        };
        let values: Vec<_> = expr_list
            .iter()
            .map(|expr| self.evaluate_expression(expr))
            .collect();
        let mut values = values.into_iter();
        for variable in variable_list {
            let value = values.next().unwrap_or(Value::Nil);
            match variable {
                Variable::Name(name) => self.scopes[0].insert(name.clone(), value),
                Variable::Selector {
                    prefix_expr,
                    selector,
                } => todo!(),
            }
        }
    }

    fn evaluate_local_variables(&mut self, statement: &Statement) {
        let Statement::LocalVariables {
            variables,
            expr_list,
        } = statement
        else {
            unreachable!("Expected local variables, found {:?}", statement);
        };
        if expr_list.is_none() {
            for variable in variables {
                self.scopes
                    .last_mut()
                    .unwrap()
                    .insert(variable.name.clone(), Value::Nil);
            }
            return;
        }
        let values: Vec<_> = expr_list
            .as_ref()
            .unwrap()
            .iter()
            .map(|expr| self.evaluate_expression(expr))
            .collect();
        let mut values = values.into_iter();
        for variable in variables {
            let value = values.next().unwrap_or(Value::Nil);
            self.scopes
                .last_mut()
                .unwrap()
                .insert(variable.name.clone(), value);
        }
    }

    fn evaluate_expression(&mut self, expression: &Expression) -> Value {
        match expression {
            Expression::Integer(n) => Value::Integer(*n),
            Expression::Float(n) => Value::Float(*n),
            Expression::True => Value::Bool(true),
            Expression::False => Value::Bool(false),
            Expression::Nil => Value::Nil,
            Expression::String(str) => Value::String(str.clone()),
            _ => todo!(),
        }
    }
}
