use std::iter::Peekable;

use crate::{
    ast::{
        Attribute, AttributeList, Block, Expression, FunctionName, Name, NameList, ParameterList,
        Statement,
    },
    error::LustError,
    lexer::Lexer,
    token::Token,
};

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

#[rustfmt::skip]
macro_rules! expect {
    ($self:ident, $token:pat_param) => {
        let token = $self.next_token()?;
        let $token = token else {
            return Err(LustError::UnexpectedToken(format!("{:?}", token)));
        };
    };
}

macro_rules! is_next {
    ($self:ident, $token:pat_param) => {
        match $self.lexer.peek() {
            Some(Ok($token)) => true,
            Some(Err(why)) => return Err(why.clone()),
            _ => false,
        }
    };
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self {
            lexer: lexer.peekable(),
        }
    }

    fn next_token(&mut self) -> Result<Token<'a>, LustError> {
        if let Some(token) = self.lexer.next() {
            Ok(token?)
        } else {
            Err(LustError::NothingToParse)
        }
    }

    fn peek_token(&mut self) -> Result<&Token<'a>, LustError> {
        if let Some(token) = self.lexer.peek() {
            token.as_ref().map_err(Clone::clone)
        } else {
            Err(LustError::NothingToParse)
        }
    }

    fn parse_statement(&mut self) -> Result<Statement<'a>, LustError> {
        match self.peek_token()? {
            Token::Semicolon => self.parse_nothing_statement(),
            Token::Break => self.parse_break_statement(),
            Token::DoubleColon => self.parse_label_statement(),
            Token::Goto => self.parse_goto_statement(),
            Token::Do => self.parse_do_statement(),
            Token::While => self.parse_while_statement(),
            Token::Repeat => self.parse_repeat_statement(),
            Token::If => self.parse_if_statement(),
            Token::For => self.parse_for_statement(),
            Token::Function => self.parse_function_statement(),
            Token::Local => self.parse_local_statement(),
            _ => Err(LustError::NotAStatement),
        }
    }

    fn parse_name(&mut self) -> Result<Name<'a>, LustError> {
        expect!(self, Token::Identifier(name));
        Ok(Name { name })
    }

    fn parse_name_list(&mut self) -> Result<NameList<'a>, LustError> {
        let mut names = NameList { names: Vec::new() };
        loop {
            let name = self.parse_name()?;
            names.push(name);
            if is_next!(self, Token::Comma) {
                expect!(self, Token::Comma);
            } else {
                break;
            }
        }

        Ok(names)
    }

    fn parse_parameter_list(&mut self) -> Result<ParameterList<'a>, LustError> {
        let mut parameters = ParameterList {
            parameters: NameList { names: Vec::new() },
            var_args: None,
        };
        #[allow(unused_must_use)]
        loop {
            match self.peek_token()? {
                Token::Identifier(name) => {
                    parameters.push(Name { name });
                    self.next_token();
                }
                Token::TripleDot => {
                    self.next_token();
                    parameters.var_args = Some(Expression::VarArgs);
                    break;
                }
                _ => break,
            }
            if is_next!(self, Token::Comma) {
                expect!(self, Token::Comma);
            } else {
                break;
            }
        }

        Ok(parameters)
    }

    fn parse_attribute(&mut self) -> Result<Attribute<'a>, LustError> {
        let name = self.parse_name()?;
        let attr = if is_next!(self, Token::LessThan) {
            expect!(self, Token::LessThan);
            let name = self.parse_name()?;
            expect!(self, Token::GreaterThan);
            Some(name)
        } else {
            None
        };
        Ok(Attribute { name, attr })
    }

    fn parse_attribute_list(&mut self) -> Result<AttributeList<'a>, LustError> {
        let mut attrs = AttributeList {
            attributes: Vec::new(),
        };
        loop {
            let attr = self.parse_attribute()?;
            attrs.push(attr);
            if is_next!(self, Token::Comma) {
                expect!(self, Token::Comma);
            } else {
                break;
            }
        }
        Ok(attrs)
    }

    fn parse_function_name(&mut self) -> Result<FunctionName<'a>, LustError> {
        // TODO: This is not right yet
        Ok(FunctionName {
            name: self.parse_name()?,
        })
    }

    fn parse_nothing_statement(&mut self) -> Result<Statement<'a>, LustError> {
        expect!(self, Token::Semicolon);
        Ok(Statement::Nothing)
    }

    fn parse_break_statement(&mut self) -> Result<Statement<'a>, LustError> {
        expect!(self, Token::Break);
        Ok(Statement::Break)
    }

    fn parse_local_statement(&mut self) -> Result<Statement<'a>, LustError> {
        expect!(self, Token::Local);
        let attrs = match self.peek_token()? {
            Token::Function => return self.parse_local_function_statement(),
            Token::Identifier(_) => self.parse_attribute_list()?,
            token => return Err(LustError::UnexpectedToken(format!("{:?}", token))),
        };

        let expressions = if is_next!(self, Token::Assign) {
            self.parse_expression_list()?
        } else {
            Vec::new()
        };

        Ok(Statement::LocalAttrs { attrs, expressions })
    }

    fn parse_local_function_statement(&mut self) -> Result<Statement<'a>, LustError> {
        expect!(self, Token::Function);
        let name = self.parse_name()?;
        expect!(self, Token::LeftParenthesis);
        let parameters = self.parse_parameter_list()?;
        expect!(self, Token::RightParenthesis);
        let block = self.parse_block()?;
        expect!(self, Token::End);

        Ok(Statement::LocalFunctionDecl {
            name,
            parameters,
            block,
        })
    }

    fn parse_function_statement(&mut self) -> Result<Statement<'a>, LustError> {
        expect!(self, Token::Function);
        let function_name = self.parse_function_name()?;
        expect!(self, Token::LeftParenthesis);
        let parameters = self.parse_parameter_list()?;
        expect!(self, Token::RightParenthesis);
        let block = self.parse_block()?;
        expect!(self, Token::End);

        Ok(Statement::FunctionDecl {
            name: function_name,
            parameters,
            block,
        })
    }

    fn parse_for_statement(&mut self) -> Result<Statement<'a>, LustError> {
        let mut names = self.parse_name_list()?;

        match self.next_token()? {
            token @ Token::Assign => if names.names.len() == 1 {
                let name = names.names.pop().expect("Should always work");
                self.parse_numeric_for(name)
            } else {
                Err(LustError::UnexpectedToken(format!("{:?}", token)))
            }
            Token::In => self.parse_generic_for(names),
            token => Err(LustError::UnexpectedToken(format!("{:?}", token))),
        }
    }

    fn parse_numeric_for(&mut self, name: Name<'a>) -> Result<Statement<'a>, LustError> {
        let start = self.parse_expression()?;
        expect!(self, Token::Comma);
        let limit = self.parse_expression()?;

        let step = match self.next_token()? {
            Token::Comma => {
                let step = self.parse_expression()?;
                expect!(self, Token::Do);
                Some(step)
            }
            Token::Do => None,
            token => return Err(LustError::UnexpectedToken(format!("{:?}", token))),
        };

        let block = self.parse_block()?;
        expect!(self, Token::End);

        Ok(Statement::NumericFor {
            start,
            limit,
            step,
            block,
            name,
        })
    }

    fn parse_generic_for(&mut self, names: NameList<'a>) -> Result<Statement<'a>, LustError> {
        let mut exprs = Vec::new();
        loop {
            let expression = self.parse_expression()?;
            exprs.push(expression);

            match self.next_token()? {
                Token::Comma => {}
                Token::Do => break,
                token => return Err(LustError::UnexpectedToken(format!("{:?}", token))),
            }
        }

        let block = self.parse_block()?;
        expect!(self, Token::End);

        Ok(Statement::GenericFor {
            names,
            exprs,
            block,
        })
    }

    fn parse_else_statement(&mut self) -> Result<Statement<'a>, LustError> {
        expect!(self, Token::Else);
        let block = self.parse_block()?;
        expect!(self, Token::End);

        Ok(Statement::Else { block })
    }

    fn parse_else_if_statement(&mut self) -> Result<Statement<'a>, LustError> {
        expect!(self, Token::Elseif);
        let condition = self.parse_expression()?;
        expect!(self, Token::Then);
        let block = self.parse_block()?;

        #[allow(unused_must_use)]
        let alternative = match self.peek_token()? {
            Token::End => {
                self.next_token();
                None
            }
            Token::Elseif => Some(Box::new(self.parse_else_if_statement()?)),
            Token::Else => Some(Box::new(self.parse_else_statement()?)),
            token => return Err(LustError::UnexpectedToken(format!("{:?}", token))),
        };

        Ok(Statement::ElseIf {
            condition,
            consequence: block,
            alternative,
        })
    }

    fn parse_if_statement(&mut self) -> Result<Statement<'a>, LustError> {
        expect!(self, Token::If);
        let condition = self.parse_expression()?;
        expect!(self, Token::Then);
        let block = self.parse_block()?;
        
        #[allow(unused_must_use)]
        let alternative = match self.peek_token()? {
            Token::End => {
                self.next_token();
                None
            }
            Token::Elseif => Some(Box::new(self.parse_else_if_statement()?)),
            Token::Else => Some(Box::new(self.parse_else_statement()?)),
            token => return Err(LustError::UnexpectedToken(format!("{:?}", token))),
        };

        Ok(Statement::If {
            condition,
            consequence: block,
            alternative,
        })
    }

    fn parse_repeat_statement(&mut self) -> Result<Statement<'a>, LustError> {
        let block = self.parse_block()?;
        expect!(self, Token::Until);
        let condition = self.parse_expression()?;

        Ok(Statement::Repeat { condition, block })
    }

    fn parse_while_statement(&mut self) -> Result<Statement<'a>, LustError> {
        let condition = self.parse_expression()?;
        expect!(self, Token::Do);
        let block = self.parse_block()?;
        expect!(self, Token::End);

        Ok(Statement::While { condition, block })
    }

    fn parse_do_statement(&mut self) -> Result<Statement<'a>, LustError> {
        let block = self.parse_block()?;
        expect!(self, Token::End);

        Ok(Statement::Do { block })
    }

    fn parse_goto_statement(&mut self) -> Result<Statement<'a>, LustError> {
        expect!(self, Token::Goto);
        expect!(self, Token::Identifier(name));

        Ok(Statement::Goto {
            name: Name { name },
        })
    }

    fn parse_label_statement(&mut self) -> Result<Statement<'a>, LustError> {
        expect!(self, Token::DoubleColon);
        expect!(self, Token::Identifier(name));
        expect!(self, Token::DoubleColon);

        Ok(Statement::Label {
            name: Name { name },
        })
    }

    fn try_parse_return_statement(&mut self) -> Result<Option<Statement<'a>>, LustError> {
        if is_next!(self, Token::Return) {
            expect!(self, Token::Return);
            let exprs = self.parse_expression_list()?;
            expect!(self, Token::Semicolon);
            Ok(Some(Statement::Return { exprs }))
        } else {
            Ok(None)
        }
    }

    fn parse_block(&mut self) -> Result<Block<'a>, LustError> {
        let mut statements = Vec::new();
        let mut return_statement = None;

        loop {
            match self.parse_statement() {
                Ok(statement) => statements.push(statement),
                Err(LustError::NotAStatement) => {
                    if let Some(ret_stat) = self.try_parse_return_statement()? {
                        return_statement = Some(Box::new(ret_stat));
                    }
                    break;
                }
                Err(LustError::NothingToParse) => break,
                Err(why) => return Err(why),
            }
        }

        Ok(Block {
            statements,
            return_statement,
        })
    }

    fn parse_expression(&mut self) -> Result<Expression<'a>, LustError> {
        todo!("Parse deez expressions")
    }

    fn parse_expression_list(&mut self) -> Result<Vec<Expression<'a>>, LustError> {
        let mut exprs = Vec::new();

        loop {
            let expr = self.parse_expression()?;
            exprs.push(expr);

            if is_next!(self, Token::Comma) {
                expect!(self, Token::Comma);
                break;
            }
        }

        Ok(exprs)
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Result<Statement<'a>, LustError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.lexer.peek().is_some() {
            Some(self.parse_statement())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn compare(code: &str, expected_statements: &[Statement]) -> Result<(), LustError> {
        let mut parser = Parser::new(Lexer::new(code));

        for (i, expected_statement) in expected_statements.into_iter().enumerate() {
            let Some(actual_statement) = parser.next() else {
                panic!("{i}: Expected {expected_statement:?} but got nothing");
            };

            assert_eq!((i, &actual_statement?), (i, expected_statement))
        }
        assert_eq!(parser.next(), None);

        Ok(())
    }

    #[test]
    fn local_attributes() -> Result<(), LustError> {
        compare(
            "local attr1, attr<ya>, trash_code",
            &[Statement::LocalAttrs {
                attrs: AttributeList {
                    attributes: vec![
                        Attribute {
                            name: Name { name: "attr1" },
                            attr: None,
                        },
                        Attribute {
                            name: Name { name: "attr" },
                            attr: Some(Name { name: "ya" }),
                        },
                        Attribute {
                            name: Name { name: "trash_code" },
                            attr: None,
                        },
                    ],
                },
                expressions: Vec::new(),
            }],
        )
    }

    #[test]
    fn empty_local_function() -> Result<(), LustError> {
        compare(
            "local function local_fn() end",
            &[Statement::LocalFunctionDecl {
                name: Name { name: "local_fn" },
                parameters: ParameterList {
                    parameters: NameList { names: Vec::new() },
                    var_args: None,
                },
                block: Block {
                    statements: Vec::new(),
                    return_statement: None,
                },
            }],
        )
    }

    #[test]
    fn empty_local_function_with_args() -> Result<(), LustError> {
        compare(
            "local function this_is_local(give, me, parameters) end",
            &[Statement::LocalFunctionDecl {
                name: Name {
                    name: "this_is_local",
                },
                parameters: ParameterList {
                    parameters: NameList {
                        names: vec![
                            Name { name: "give" },
                            Name { name: "me" },
                            Name { name: "parameters" },
                        ],
                    },
                    var_args: None,
                },
                block: Block {
                    statements: Vec::new(),
                    return_statement: None,
                },
            }],
        )
    }

    #[test]
    fn empty_local_function_var_args() -> Result<(), LustError> {
        compare(
            "local function put_everything_in_me_but_locally(...) end",
            &[Statement::LocalFunctionDecl {
                name: Name {
                    name: "put_everything_in_me_but_locally",
                },
                parameters: ParameterList {
                    parameters: NameList { names: Vec::new() },
                    var_args: Some(Expression::VarArgs),
                },
                block: Block {
                    statements: Vec::new(),
                    return_statement: None,
                },
            }],
        )
    }

    #[test]
    fn empty_local_function_with_args_and_var_args() -> Result<(), LustError> {
        compare(
            "local function the_famous_do_stuff_function(what, is, this, supposed, ...) end",
            &[Statement::LocalFunctionDecl {
                name: Name {
                    name: "the_famous_do_stuff_function",
                },
                parameters: ParameterList {
                    parameters: NameList {
                        names: vec![
                            Name { name: "what" },
                            Name { name: "is" },
                            Name { name: "this" },
                            Name { name: "supposed" },
                        ],
                    },
                    var_args: Some(Expression::VarArgs),
                },
                block: Block {
                    statements: Vec::new(),
                    return_statement: None,
                },
            }],
        )
    }

    #[test]
    fn empty_function() -> Result<(), LustError> {
        compare(
            "function fn() end",
            &[Statement::FunctionDecl {
                name: FunctionName {
                    name: Name { name: "fn" },
                },
                parameters: ParameterList {
                    parameters: NameList { names: Vec::new() },
                    var_args: None,
                },
                block: Block {
                    statements: Vec::new(),
                    return_statement: None,
                },
            }],
        )
    }

    #[test]
    fn empty_function_with_args() -> Result<(), LustError> {
        compare(
            "function my_beautiful_10th_function(a, bc, DEF, a_b_c) end",
            &[Statement::FunctionDecl {
                name: FunctionName {
                    name: Name {
                        name: "my_beautiful_10th_function",
                    },
                },
                parameters: ParameterList {
                    parameters: NameList {
                        names: vec![
                            Name { name: "a" },
                            Name { name: "bc" },
                            Name { name: "DEF" },
                            Name { name: "a_b_c" },
                        ],
                    },
                    var_args: None,
                },
                block: Block {
                    statements: Vec::new(),
                    return_statement: None,
                },
            }],
        )
    }

    #[test]
    fn empty_function_var_args() -> Result<(), LustError> {
        compare(
            "function put_everything_in_me(...) end",
            &[Statement::FunctionDecl {
                name: FunctionName {
                    name: Name {
                        name: "put_everything_in_me",
                    },
                },
                parameters: ParameterList {
                    parameters: NameList { names: Vec::new() },
                    var_args: Some(Expression::VarArgs),
                },
                block: Block {
                    statements: Vec::new(),
                    return_statement: None,
                },
            }],
        )
    }

    #[test]
    fn empty_function_with_args_and_var_args() -> Result<(), LustError> {
        compare(
            "function give_your_things(anil, falsy, ...) end",
            &[Statement::FunctionDecl {
                name: FunctionName {
                    name: Name {
                        name: "give_your_things",
                    },
                },
                parameters: ParameterList {
                    parameters: NameList {
                        names: vec![Name { name: "anil" }, Name { name: "falsy" }],
                    },
                    var_args: Some(Expression::VarArgs),
                },
                block: Block {
                    statements: Vec::new(),
                    return_statement: None,
                },
            }],
        )
    }

    #[test]
    fn simple_statements() -> Result<(), LustError> {
        compare(";", &[Statement::Nothing])?;
        compare(
            "::label::",
            &[Statement::Label {
                name: Name { name: "label" },
            }],
        )?;
        compare(
            "goto label",
            &[Statement::Goto {
                name: Name { name: "label" },
            }],
        )?;
        compare("break", &[Statement::Break])?;

        Ok(())
    }
}
