use crate::{
    ast::{Block, Expression, Statement},
    error::LustError,
    lexer::Lexer,
    token::Token,
};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
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

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self { lexer }
    }

    fn next_token(&mut self) -> Result<Token<'a>, LustError> {
        let Some(token) = self.lexer.next() else {
            return Err(LustError::NothingToParse);
        };

        Ok(token?)
    }

    fn parse_statement(&mut self) -> Result<Statement<'a>, LustError> {
        let statement = match self.next_token()? {
            Token::Semicolon => Statement::Nothing,
            Token::Break => Statement::Break,
            Token::DoubleColon => self.parse_label_statement()?,
            Token::Goto => self.parse_goto_statement()?,
            Token::Do => self.parse_do_statement()?,
            Token::While => self.parse_while_statement()?,
            Token::Repeat => self.parse_repeat_statement()?,
            Token::If => self.parse_if_statement()?,
            Token::For => self.parse_for_statement()?,
            token => unimplemented!("Probably not implemented yet: {:?}", token),
        };

        Ok(statement)
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
            var: todo!("variables"),
        })
    }

    fn parse_generic_for(
        &mut self,
        first: &'a str,
        vars_done: bool,
    ) -> Result<Statement<'a>, LustError> {
        if !vars_done {
            loop {
                match self.next_token()? {
                    Token::Comma => {}
                    Token::In => break,
                    token => return Err(LustError::UnexpectedToken(format!("{:?}", token))),
                };

                todo!("variables aaaaaaaaaaaaa")
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
            vars: todo!("variables mah guy"),
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
        expect!(self, Token::Identifier(identifier));

        Ok(Statement::Goto { identifier })
    }

    fn parse_label_statement(&mut self) -> Result<Statement<'a>, LustError> {
        expect!(self, Token::Identifier(identifier));
        expect!(self, Token::DoubleColon);

        Ok(Statement::Label { identifier })
    }

    fn parse_block(&mut self) -> Result<Block<'a>, LustError> {
        todo!("Parse deez blocks")
    }

    fn parse_expression(&mut self) -> Result<Expression, LustError> {
        todo!("Parse deez expressions")
    }
}
