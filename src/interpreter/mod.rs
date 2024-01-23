use std::collections::HashMap;

use crate::parser::{
    expression::Expression,
    statement::{Block, Parameters, Statement, Variable},
};

#[derive(Debug)]
pub struct Interpreter<'a> {
    pub scopes: Vec<Scope<'a>>,
}

#[derive(Debug)]
pub struct Scope<'a> {
    block: &'a Block,
    table: HashMap<String, Value>,
    labels: HashMap<String, usize>,
}

impl<'a> Scope<'a> {
    fn new(block: &'a Block) -> Self {
        Self {
            block,
            table: HashMap::new(),
            labels: HashMap::new(),
        }
    }

    fn look_for_labels(&mut self) {
        for (i, statement) in self.block.statements.iter().enumerate() {
            if let Statement::Label(name) = statement {
                self.labels.insert(name.clone(), i);
            }
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
    pub fn new() -> Self {
        Self { scopes: vec![] }
    }

    pub fn interpret(&mut self, block: &'a Block) {
        if let Some(name) = self.evaluate_block(block) {
            todo!("No visible label '{name}' for <goto>");
        }
    }

    fn evaluate_block(&mut self, block: &'a Block) -> Option<String> {
        let mut scope = Scope::new(block);
        scope.look_for_labels();
        self.scopes.push(scope);

        let mut i = 0;
        while i < block.statements.len() {
            let statement = &block.statements[i];
            match statement {
                Statement::LocalVariables { .. } => self.evaluate_local_variables(statement),
                Statement::Assignment { .. } => self.evaluate_global_variables(statement),
                Statement::Do(_) => {
                    if let Some(name) = self.evaluate_do(statement) {
                        if let Some(pos) = self.scopes.last().unwrap().labels.get(&name) {
                            i = *pos;
                        } else {
                            let scope = self.scopes.pop().unwrap();
                            println!("Scope after: {:?}", scope);
                            return Some(name);
                        }
                    }
                }
                Statement::Goto(name) => {
                    if let Some(pos) = self.scopes.last().unwrap().labels.get(name) {
                        i = *pos;
                    } else {
                        let scope = self.scopes.pop().unwrap();
                        println!("Scope after: {:?}", scope);
                        return Some(name.clone());
                    }
                }
                Statement::Label(_) => {}
                Statement::Empty => {}
                Statement::Break => todo!("Break outside a loop"),
                _ => unreachable!("Expected statement, found {:?}", statement),
            }
            i += 1;
        }
        println!("[END] Scope after: {:?}", self.scopes.last().unwrap());
        self.scopes.pop();
        return None;
    }

    fn evaluate_do(&mut self, statement: &'a Statement) -> Option<String> {
        let Statement::Do(block) = statement else {
            unreachable!("Expected do statement, found {:?}", statement);
        };
        return self.evaluate_block(block);
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
                Variable::Selector { .. } => todo!(),
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
