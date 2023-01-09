use std::iter::Peekable;

use crate::{
    ast::{
        Attribute, AttributeList, Block, Expression, FunctionName, Name, NameList, ParameterList,
        Statement, Variable,
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
        match $self.peek_token()? {
            pat_param => {
                $self.next_token();
                true
            }
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

    fn parse_statement(&mut self, token: Token) -> Result<Statement<'a>, LustError> {
        let statement = match token {
            Token::Semicolon => Statement::Nothing,
            Token::Break => Statement::Break,
            Token::DoubleColon => self.parse_label_statement()?,
            Token::Goto => self.parse_goto_statement()?,
            Token::Do => self.parse_do_statement()?,
            Token::While => self.parse_while_statement()?,
            Token::Repeat => self.parse_repeat_statement()?,
            Token::If => self.parse_if_statement()?,
            Token::For => self.parse_for_statement()?,
            Token::Function => self.parse_function_statement()?,
            Token::Local => self.parse_local_statement()?,
            token => unimplemented!("Probably not implemented yet: {:?}", token),
        };

        Ok(statement)
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
            match self.peek_token()? {
                Token::Comma => {
                    self.next_token();
                }
                _ => break,
            }
        }

        Ok(names)
    }

    fn parse_attribute(&mut self) -> Result<Attribute<'a>, LustError> {
        expect!(self, Token::Identifier(name));
        let attr = if is_next!(self, Token::LessThan) {
            expect!(self, Token::Identifier(name));
            Some(name)
        } else {
            None
        };

        Ok(Attribute {
            name: Name { name },
            attr: attr.map(|name| Name { name }),
        })
    }

    fn parse_function_name(&mut self) -> Result<FunctionName<'a>, LustError> {
        Ok(FunctionName {
            name: self.parse_name()?,
        })
    }

    fn parse_local_statement(&mut self) -> Result<Statement<'a>, LustError> {
        let first = match self.peek_token()? {
            Token::Function => return self.parse_function_statement(),
            Token::Identifier(_) => self.parse_attribute()?,
            token => return Err(LustError::UnexpectedToken(format!("{:?}", token))),
        };

        let mut attrs = AttributeList {
            attributes: Vec::new(),
        };
        attrs.push(first);
        loop {
            match self.peek_token()? {
                Token::Comma => {
                    self.next_token();
                }
                Token::Assign => {
                    self.next_token();
                    break;
                }
                _ => {
                    return Ok(Statement::LocalAttrs {
                        attrs,
                        expressions: Vec::new(),
                    })
                }
            };
        }

        let mut expressions = Vec::new();
        loop {
            let expr = self.parse_expression()?;
            expressions.push(expr);
            match self.peek_token()? {
                Token::Comma => {
                    self.next_token();
                }
                _ => break,
            }
        }

        Ok(Statement::LocalAttrs { attrs, expressions })
    }

    fn parse_function_statement(&mut self) -> Result<Statement<'a>, LustError> {
        let function_name = self.parse_function_name()?;
        expect!(self, Token::LeftParenthesis);

        let mut parameters = ParameterList {
            parameters: NameList { names: Vec::new() },
            var_args: None,
        };
        loop {
            match self.next_token()? {
                Token::RightParenthesis => break,
                Token::Identifier(name) => parameters.push(Name { name }),
                Token::TripleDot => {
                    parameters.var_args = Some(Expression::VarArgs);
                    break;
                }
                token => return Err(LustError::UnexpectedToken(format!("{:?}", token))),
            };
        }

        let block = self.parse_block()?;
        expect!(self, Token::End);

        Ok(Statement::FunctionDecl {
            name: function_name,
            parameters,
            block,
        })
    }

    fn parse_for_statement(&mut self) -> Result<Statement<'a>, LustError> {
        expect!(self, Token::Identifier(identifier));

        match self.next_token()? {
            Token::Assign => self.parse_numeric_for(identifier),
            Token::Comma => self.parse_generic_for(identifier, false),
            Token::In => self.parse_generic_for(identifier, true),
            token => Err(LustError::UnexpectedToken(format!("{:?}", token))),
        }
    }

    fn parse_numeric_for(&mut self, first: &'a str) -> Result<Statement<'a>, LustError> {
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
            var: Name { name: first },
        })
    }

    fn parse_generic_for(
        &mut self,
        first: &'a str,
        vars_done: bool,
    ) -> Result<Statement<'a>, LustError> {
        let mut vars = NameList { names: Vec::new() };

        vars.push(Name { name: first });
        if !vars_done {
            loop {
                match self.next_token()? {
                    Token::Comma => {}
                    Token::In => break,
                    token => return Err(LustError::UnexpectedToken(format!("{:?}", token))),
                };

                expect!(self, Token::Identifier(name));
                vars.push(Name { name });
            }
        }

        let mut expressions = Vec::new();
        loop {
            let expression = self.parse_expression()?;
            expressions.push(expression);

            match self.next_token()? {
                Token::Comma => {}
                Token::Do => break,
                token => return Err(LustError::UnexpectedToken(format!("{:?}", token))),
            }
        }

        let block = self.parse_block()?;
        expect!(self, Token::End);

        Ok(Statement::GenericFor {
            vars,
            exprs: expressions,
            block,
        })
    }

    fn parse_if_statement(&mut self) -> Result<Statement<'a>, LustError> {
        let condition = self.parse_expression()?;
        expect!(self, Token::Then);
        let block = self.parse_block()?;

        let alternative = match self.next_token()? {
            Token::End => None,
            Token::Elseif => Some(self.parse_if_statement()?),
            Token::Else => Some(self.parse_do_statement()?), // TODO: Find a better way, c'mon
            token => return Err(LustError::UnexpectedToken(format!("{:?}", token))),
        };

        Ok(Statement::If {
            condition,
            consequence: block,
            alternative: Box::new(alternative),
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
        expect!(self, Token::Identifier(name));

        Ok(Statement::Goto {
            name: Name { name },
        })
    }

    fn parse_label_statement(&mut self) -> Result<Statement<'a>, LustError> {
        expect!(self, Token::Identifier(name));
        expect!(self, Token::DoubleColon);

        Ok(Statement::Label {
            name: Name { name },
        })
    }

    fn parse_block(&mut self) -> Result<Block<'a>, LustError> {
        todo!("Parse deez blocks")
    }

    fn parse_expression(&mut self) -> Result<Expression<'a>, LustError> {
        todo!("Parse deez expressions")
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Result<Statement<'a>, LustError>;

    fn next(&mut self) -> Option<Self::Item> {
        let token = match self.lexer.next()? {
            Ok(token) => token,
            Err(why) => return Some(Err(why)),
        };

        Some(self.parse_statement(token))
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

    // #[test]
    // fn empty_function() -> Result<(), LustError> {
    //     compare(
    //         "function fn() end",
    //         &[Statement::FunctionDecl {
    //             expression: Expression::Variable(Variable::new("fn")),
    //             args: Vec::new(),
    //             block: Block { statements: Vec::new() },
    //         }],
    //     )
    // }

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
