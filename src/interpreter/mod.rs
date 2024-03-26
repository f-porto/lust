pub mod value;

use std::{collections::HashMap, error::Error, mem, vec};

use crate::parser::{
    expression::Expression,
    prefix_expression::{Argument, CallSuffix, PExprAction, PrefixExpression, Primary, Selector},
    statement::{Block, FunctionName, If, Parameters, Statement, Variable},
};

use self::value::{Table, Value};

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

    pub fn insert(&mut self, name: String, value: Value) {
        self.table.insert(name, value);
    }

    fn get(&self, name: &str) -> Option<&Value> {
        self.table.get(name)
    }

    fn get_mut(&mut self, name: &str) -> Option<&mut Value> {
        self.table.get_mut(name)
    }
}

#[derive(Debug)]
enum Command {
    Continue,
    Goto(String),
    Break,
    Error(Box<dyn Error>),
}

impl<'a> Interpreter<'a> {
    pub fn new() -> Self {
        Self { scopes: vec![] }
    }

    pub fn interpret(&mut self, block: &'a Block) {
        let global_block = Block {
            statements: vec![],
            return_statement: None,
        };
        let global_block = Box::new(global_block);
        let global_block = Box::leak(global_block);
        let mut scope = Scope::new(global_block);
        crate::std::load_std(&mut scope);
        self.scopes.push(scope);
        match self.evaluate_block(block) {
            Command::Goto(name) => todo!("No visible label '{name}' for <goto>"),
            Command::Break => todo!("Break outside a loop"),
            Command::Error(err) => todo!("Error happened: {err}"),
            _ => {}
        }
        self.scopes.pop();
    }

    fn evaluate_block(&mut self, block: &'a Block) -> Command {
        let mut scope = Scope::new(block);
        scope.look_for_labels();
        self.scopes.push(scope);

        let mut i = 0;
        while i < block.statements.len() {
            let statement = &block.statements[i];
            let label = match statement {
                Statement::LocalVariables { .. } => self.evaluate_local_variables(statement),
                Statement::Assignment { .. } => self.evaluate_global_variables(statement),
                Statement::Do(_) => self.evaluate_do(statement),
                Statement::If { .. } => self.evaluate_if(statement),
                Statement::While { .. } => self.evaluate_while(statement),
                Statement::Repeat { .. } => self.evaluate_repeat(statement),
                Statement::LocalFunctionDefinition { .. } => {
                    self.evaluate_local_function_definition(statement)
                }
                Statement::FunctionDefinition { .. } => {
                    self.evaluate_global_function_definition(statement)
                }
                Statement::NumericalFor { .. } => self.evaluate_numerical_for(statement),
                Statement::FunctionCall { .. } => self.evaluate_function_call(statement),
                Statement::Goto(name) => Command::Goto(name.clone()),
                Statement::Label(_) => Command::Continue,
                Statement::Empty => Command::Continue,
                Statement::Break => Command::Break,
                _ => unreachable!("Expected statement, found {:?}", statement),
            };
            match label {
                Command::Goto(name) => {
                    if let Some(pos) = self.scopes.last().unwrap().labels.get(&name) {
                        i = *pos;
                    } else {
                        let scope = self.scopes.pop().unwrap();
                        println!("Scope after: {:?}", scope);
                        return Command::Goto(name);
                    }
                }
                Command::Break => {
                    self.scopes.pop();
                    return Command::Break;
                }
                Command::Error(err) => {
                    self.scopes.pop();
                    return Command::Error(err);
                }
                _ => {}
            }
            i += 1;
        }
        // println!("[END] Scope after: {:?}", self.scopes.last().unwrap());
        self.scopes.pop();
        Command::Continue
    }

    fn evaluate_function_call(&mut self, statement: &'a Statement) -> Command {
        let Statement::FunctionCall { prefix_exp, call } = statement else {
            unreachable!("Expected function call, found: {:?}", statement);
        };
        let Primary::Name(name) = &prefix_exp.primary else {
            todo!("Deal with expressions")
        };
        let Some(object) = self.get(name) else {
            todo!("Error if variable is not found");
        };
        if !matches!(object, Value::Builtin(_) | Value::Lambda { .. }) {
            todo!("Error if variable is not a function");
        }
        let arg = match call {
            CallSuffix::Simple(arg) => arg,
            CallSuffix::Method { .. } => todo!("How to call methods"),
        };
        let values = match arg {
            Argument::List(args) => args.iter().map(|x| self.evaluate_expression(x)).collect(),
            Argument::String(s) => vec![Value::String(s.clone())],
            Argument::Table(t) => vec![self.evaluate_expression(t)],
        };
        let object = self.get(name).unwrap().clone();
        match object {
            Value::Builtin(f) => f.call(values),
            Value::Lambda { parameters, body } => {
                let body = Box::leak(Box::new(body));
                let block = Box::leak(Box::new(Block::default()));
                let mut scope = Scope::new(block);
                let Parameters { name_list, var_arg } = parameters;
                for (name, value) in name_list.into_iter().zip(values) {
                    scope.insert(name, value)
                }

                self.scopes.push(scope);
                self.evaluate_block(body);
                self.scopes.pop();
                Value::Nil
            }
            o => unreachable!("Expected function, found {:?}", o),
        };
        Command::Continue
    }

    fn evaluate_global_function_definition(&mut self, statement: &'a Statement) -> Command {
        let Statement::FunctionDefinition {
            function_name,
            parameters,
            body,
        } = statement
        else {
            unreachable!("Expected function definition, found {:?}", statement);
        };
        let parameters = parameters.clone().unwrap_or(Parameters {
            name_list: vec![],
            var_arg: false,
        });
        let lambda = Value::Lambda {
            parameters,
            body: body.clone(),
        };
        let FunctionName { names, method } = function_name;
        if let Some(method) = method {
            self.add_method_to_table(&names[0], &names[1..], method, lambda)
        } else if names.len() == 1 {
            self.add_function(&names[0], lambda);
        } else {
            let last = names.len() - 1;
            let members = &names[1..last];
            self.add_function_to_table(&names[0], members, &names[last], lambda);
        }
        Command::Continue
    }

    fn add_function(&mut self, function_name: &str, body: Value) {
        self.scopes[0].insert(function_name.to_string(), body)
    }

    fn add_function_to_table(
        &mut self,
        table_name: &str,
        members: &[String],
        function_name: &str,
        body: Value,
    ) {
        let value = self.get_mut(table_name);
        let Some(Value::Table(t)) = value else {
            return;
        };
        let mut table = t;
        for member in members.iter() {
            let value = table.get_mut(&Value::String(member.to_string()));
            let Some(Value::Table(t)) = value else {
                return;
            };
            table = t;
        }
        table.insert(&Value::String(function_name.to_string()), &body);
    }

    fn add_method_to_table(
        &mut self,
        table_name: &str,
        members: &[String],
        method_name: &str,
        body: Value,
    ) {
        let value = self.get_mut(table_name);
        let Some(Value::Table(t)) = value else {
            return;
        };
        let mut table = t;
        for member in members.iter() {
            let value = table.get_mut(&Value::String(member.to_string()));
            let Some(Value::Table(t)) = value else {
                return;
            };
            table = t;
        }
        let Value::Lambda {
            mut parameters,
            body,
        } = body
        else {
            unreachable!("Expected lambda, found {:?}", body);
        };
        parameters.name_list.insert(0, "self".to_string());
        let body = Value::Lambda { parameters, body };
        table.insert(&Value::String(method_name.to_string()), &body);
    }

    fn evaluate_local_function_definition(&mut self, statement: &'a Statement) -> Command {
        let Statement::LocalFunctionDefinition {
            name,
            parameters,
            body,
        } = statement
        else {
            unreachable!("Expected function definition, found {:?}", statement);
        };
        let parameters = parameters.clone().unwrap_or(Parameters {
            name_list: vec![],
            var_arg: false,
        });
        let lambda = Value::Lambda {
            parameters,
            body: body.clone(),
        };
        self.scopes.last_mut().unwrap().insert(name.clone(), lambda);
        Command::Continue
    }

    fn evaluate_numerical_for(&mut self, statement: &'a Statement) -> Command {
        let Statement::NumericalFor {
            control,
            initial,
            limit,
            step,
            block,
        } = statement
        else {
            unreachable!("Expected numerical for, found {:?}", statement);
        };
        let initial = self.evaluate_expression(initial);
        let limit = self.evaluate_expression(limit);
        let step = step
            .as_ref()
            .map(|x| self.evaluate_expression(x))
            .unwrap_or(Value::Integer(1));

        let mut initial = match initial.to_number() {
            Ok(v) => v,
            Err(e) => return Command::Error(e),
        };
        let mut limit = match limit.to_number() {
            Ok(v) => v,
            Err(e) => return Command::Error(e),
        };
        let mut step = match step.to_number() {
            Ok(v) => v,
            Err(e) => return Command::Error(e),
        };
        if !matches!(
            (&initial, &limit, &step),
            (Value::Integer(_), Value::Integer(_), Value::Integer(_))
        ) {
            initial = initial.to_float().unwrap();
            limit = limit.to_float().unwrap();
            step = step.to_float().unwrap();
        }
        let limit = limit;
        let step = step;

        let for_block = Block {
            statements: vec![],
            return_statement: None,
        };
        let for_block = Box::new(for_block);
        let ref_block: &'static _ = Box::leak(for_block);
        let mut scope = Scope::new(&ref_block);
        scope.insert(control.clone(), initial);
        self.scopes.push(scope);
        let index = self.scopes.len() - 1;

        let mut i = self.get_from(&control, index).unwrap();
        while i.is_less_or_equal(&limit).unwrap() {
            self.evaluate_block(block);

            let initial = self.get_mut_from(&control, index).unwrap();
            let r = initial.add(&step).unwrap();
            let _ = mem::replace(initial, r);

            i = self.get_from(&control, index).unwrap();
        }

        self.scopes.pop();
        Command::Continue
    }

    fn evaluate_while(&mut self, statement: &'a Statement) -> Command {
        let Statement::While { condition, block } = statement else {
            unreachable!("Expected while statement, found {:?}", statement);
        };
        while self.evaluate_expression(condition).is_truthy() {
            let label = self.evaluate_block(block);
            match label {
                Command::Goto(_) => return label,
                Command::Break => break,
                Command::Error(_) => return label,
                _ => {}
            }
        }
        Command::Continue
    }

    fn evaluate_repeat(&mut self, statement: &'a Statement) -> Command {
        let Statement::Repeat { block, condition } = statement else {
            unreachable!("Expected repeat statement, found {:?}", statement);
        };
        let label = self.evaluate_block(block);
        match label {
            Command::Goto(_) => return label,
            Command::Break => return Command::Continue,
            _ => {}
        }
        while !self.evaluate_expression(condition).is_truthy() {
            let label = self.evaluate_block(block);
            match label {
                Command::Goto(_) => return label,
                Command::Break => break,
                Command::Error(_) => return label,
                _ => {}
            }
        }
        Command::Continue
    }

    fn evaluate_if(&mut self, statement: &'a Statement) -> Command {
        let Statement::If { ifs, r#else } = statement else {
            unreachable!("Expected if statement, found {:?}", statement);
        };
        for r#if in ifs {
            let If { condition, block } = r#if;
            let result = self.evaluate_expression(condition);
            if result.is_truthy() {
                return self.evaluate_block(block);
            }
        }
        let Some(block) = r#else else {
            return Command::Continue;
        };
        self.evaluate_block(block)
    }

    fn evaluate_do(&mut self, statement: &'a Statement) -> Command {
        let Statement::Do(block) = statement else {
            unreachable!("Expected do statement, found {:?}", statement);
        };
        return self.evaluate_block(block);
    }

    fn evaluate_global_variables(&mut self, statement: &Statement) -> Command {
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
        Command::Continue
    }

    fn evaluate_local_variables(&mut self, statement: &Statement) -> Command {
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
            return Command::Continue;
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
        Command::Continue
    }

    fn evaluate_expression(&mut self, expression: &Expression) -> Value {
        match expression {
            Expression::Integer(n) => Value::Integer(*n),
            Expression::Float(n) => Value::Float(*n),
            Expression::True => Value::True,
            Expression::False => Value::False,
            Expression::Nil => Value::Nil,
            Expression::String(str) => Value::String(str.clone()),
            Expression::Lambda { parameters, body } => Value::Lambda {
                parameters: parameters.clone().unwrap_or(Parameters {
                    name_list: vec![],
                    var_arg: false,
                }),
                body: body.clone(),
            },
            Expression::Table(fields) => {
                Value::Table(Table::from_fields(fields, |e| self.evaluate_expression(e)))
            }
            _ => todo!("Evaluate expression: {:?}", expression),
        }
    }

    fn evaluate_expression_for_reference(&mut self, expression: &Expression) -> Option<&mut Value> {
        match expression {
            Expression::PrefixExpression(p) => self.evaluate_prefix_expression(p),
            Expression::Lambda { parameters, body } => todo!(),
            _ => None,
        }
    }

    fn evaluate_prefix_expression(&mut self, prefix_expr: &PrefixExpression) -> Option<&mut Value> {
        todo!("Damm fucking mutability");
        let PrefixExpression { primary, actions } = prefix_expr;
        let mut reference = match primary {
            Primary::Name(name) => self.get_mut(name),
            Primary::Expression(expression) => self.evaluate_expression_for_reference(expression),
        }?;
        for action in actions.iter() {
            let x = match action {
                PExprAction::Selector(selector) => match selector {
                    Selector::Dot(name) => Value::String(name.to_string()),
                    Selector::Key(expr) => self.evaluate_expression(expr),
                },
                PExprAction::Call(call) => match call {
                    CallSuffix::Simple(arg) => todo!(),
                    CallSuffix::Method { name, argument } => todo!(),
                },
            };
            let Value::Table(table) = reference else {
                return None;
            };
            reference = table.get_mut(&x)?;
        }
        Some(reference)
    }

    fn get_from(&self, name: &str, index: usize) -> Option<&Value> {
        self.scopes[index].get(name)
    }

    fn get_mut_from(&mut self, name: &str, index: usize) -> Option<&mut Value> {
        self.scopes[index].get_mut(name)
    }

    fn get(&self, name: &str) -> Option<&Value> {
        let mut value = None;
        for scope in self.scopes.iter().rev() {
            value = scope.get(name);
            if value.is_some() {
                break;
            }
        }
        value
    }

    fn get_mut(&mut self, name: &str) -> Option<&mut Value> {
        let mut value = None;
        for scope in self.scopes.iter_mut().rev() {
            value = scope.get_mut(name);
            if value.is_some() {
                break;
            }
        }
        value
    }
}
