#[derive(Debug)]
pub enum Statement<'a> {
    Nothing,
    Assignment,
    FunctionCall,
    Label { identifier: &'a str },
    Break,
    Goto { identifier: &'a str },
    Do { block: Block<'a> },
    While {
        condition: Expression,
        block: Block<'a>,
    },
    Repeat {
        condition: Expression,
        block: Block<'a>,
    },
    If {
        condition: Expression,
        consequence: Block<'a>,
        alternative: Box<Option<Statement<'a>>>,
    },
    NumericFor {
        var: Expression,
        start: Expression,
        limit: Expression,
        step: Option<Expression>,
        block: Block<'a>,
    },
    GenericFor {
        vars: Vec<Expression>,
        exprs: Vec<Expression>,
        block: Block<'a>,
    },
    FunctionDecl,
    LocalFunctionDecl,
    LocalAttrs,
}

#[derive(Debug)]
pub struct Block<'a> {
    statements: Vec<Statement<'a>>,
}

#[derive(Debug)]
pub enum Expression {
    Variable {
        value: String,
    },
    Number {
        value: isize,
    },
    Boolean {
        value: bool,
    },
    Addition {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Subtraction {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Multiplication {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Division {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    IsGreaterThan {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    IsLessThan {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    IsGreaterOrEqual {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    IsLessOrEqual {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    IsEqual {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    IsNotEqual {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Negate {
        value: Box<Expression>,
    },
    ToggleSignal {
        value: Box<Expression>,
    },
    KeepSignal {
        value: Box<Expression>,
    },
}
