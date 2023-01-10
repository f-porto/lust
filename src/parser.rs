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
        match $self.peek_token()? {
            $token => {
                $self.next_token()?; // This fucking stupid warning
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
            if !is_next!(self, Token::Comma) {
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

        Ok(parameters)
    }

    fn parse_attribute(&mut self) -> Result<Attribute<'a>, LustError> {
        expect!(self, Token::Identifier(name));
        let attr = if is_next!(self, Token::LessThan) {
            expect!(self, Token::Identifier(name));
            expect!(self, Token::GreaterThan);
            Some(name)
        } else {
            None
        };

        Ok(Attribute {
            name: Name { name },
            attr: attr.map(|name| Name { name }),
        })
    }

    fn parse_attribute_list(&mut self) -> Result<AttributeList<'a>, LustError> {
        let mut attrs = AttributeList {
            attributes: Vec::new(),
        };
        loop {
            let attr = self.parse_attribute()?;
            attrs.push(attr);
            if !is_next!(self, Token::Comma) {
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
        let attrs = match self.peek_token()? {
            Token::Function => return self.parse_function_statement(),
            Token::Identifier(_) => self.parse_attribute_list()?,
            token => return Err(LustError::UnexpectedToken(format!("{:?}", token))),
        };

        let expressions = self.parse_expression_list()?;

        Ok(Statement::LocalAttrs { attrs, expressions })
    }

    fn parse_function_statement(&mut self) -> Result<Statement<'a>, LustError> {
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
            Token::Assign => self.parse_numeric_for(names.names.pop().expect("This sucks")), // TODO: This sucks, do better
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

    fn parse_if_statement(&mut self) -> Result<Statement<'a>, LustError> {
        let condition = self.parse_expression()?;
        expect!(self, Token::Then);
        let block = self.parse_block()?;

        let alternative = match self.next_token()? {
            Token::End => None,
            Token::Elseif => Some(Box::new(self.parse_if_statement()?)),
            Token::Else => Some(Box::new(self.parse_do_statement()?)), // TODO: Find a better way, c'mon
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
            let exprs = self.parse_expression_list()?;
            is_next!(self, Token::Semicolon);
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

        Ok(Block { statements, return_statement })
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
