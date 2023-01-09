#[derive(Debug, PartialEq)]
pub enum Statement<'a> {
    Nothing,
    Assignment {
        vars: Vec<Expression<'a>>,
        expressions: Vec<Expression<'a>>,
    },
    FunctionCall {
        expression: Expression<'a>,
        args: Vec<Expression<'a>>,
    },
    Label {
        identifier: &'a str,
    },
    Break,
    Goto {
        identifier: &'a str,
    },
    Do {
        block: Block<'a>,
    },
    While {
        condition: Expression<'a>,
        block: Block<'a>,
    },
    Repeat {
        condition: Expression<'a>,
        block: Block<'a>,
    },
    If {
        condition: Expression<'a>,
        consequence: Block<'a>,
        alternative: Box<Option<Statement<'a>>>,
    },
    NumericFor {
        var: Expression<'a>,
        start: Expression<'a>,
        limit: Expression<'a>,
        step: Option<Expression<'a>>,
        block: Block<'a>,
    },
    GenericFor {
        vars: Vec<Expression<'a>>,
        exprs: Vec<Expression<'a>>,
        block: Block<'a>,
    },
    FunctionDecl {
        expression: Expression<'a>,
        args: Vec<Expression<'a>>,
    },
    LocalFunctionDecl {
        expression: Expression<'a>,
        args: Vec<Expression<'a>>,
    },
    LocalAttrs {
        attrs: Vec<Expression<'a>>,
        expressions: Vec<Expression<'a>>,
    },
}

#[derive(Debug, PartialEq)]
pub struct Block<'a> {
    statements: Vec<Statement<'a>>,
}

#[derive(Debug, PartialEq)]
pub enum Expression<'a> {
    Nil,
    Bool(bool),
    Number(Number),
    String(&'a str),
    Table(Table),
    VarArgs,
    FunctionDef {
        args: Vec<Expression<'a>>,
        block: Block<'a>,
    },
    Variable(Variable),
    FunctionCall {
        expression: Box<Expression<'a>>,
        args: Vec<Expression<'a>>,
    },

    UnaryMinus {
        expression: Box<Expression<'a>>,
    },
    BitwiseNot {
        expression: Box<Expression<'a>>,
    },
    Length {
        expression: Box<Expression<'a>>,
    },
    Not {
        expression: Box<Expression<'a>>,
    },

    Addition {
        lhs: Box<Expression<'a>>,
        rhs: Box<Expression<'a>>,
    },
    Subtraction {
        lhs: Box<Expression<'a>>,
        rhs: Box<Expression<'a>>,
    },
    Multiplication {
        lhs: Box<Expression<'a>>,
        rhs: Box<Expression<'a>>,
    },
    Division {
        lhs: Box<Expression<'a>>,
        rhs: Box<Expression<'a>>,
    },
    FloorDivision {
        lhs: Box<Expression<'a>>,
        rhs: Box<Expression<'a>>,
    },
    Exponentiation {
        lhs: Box<Expression<'a>>,
        rhs: Box<Expression<'a>>,
    },
    Modulo {
        lhs: Box<Expression<'a>>,
        rhs: Box<Expression<'a>>,
    },

    BitwiseAnd {
        lhs: Box<Expression<'a>>,
        rhs: Box<Expression<'a>>,
    },
    BitwiseOr {
        lhs: Box<Expression<'a>>,
        rhs: Box<Expression<'a>>,
    },
    BitwiseXor {
        lhs: Box<Expression<'a>>,
        rhs: Box<Expression<'a>>,
    },
    BitwiseLeftShift {
        lhs: Box<Expression<'a>>,
        rhs: Box<Expression<'a>>,
    },
    BitwiseRightShift {
        lhs: Box<Expression<'a>>,
        rhs: Box<Expression<'a>>,
    },

    Concatenation {
        lhs: Box<Expression<'a>>,
        rhs: Box<Expression<'a>>,
    },

    GreaterThan {
        lhs: Box<Expression<'a>>,
        rhs: Box<Expression<'a>>,
    },
    LessThan {
        lhs: Box<Expression<'a>>,
        rhs: Box<Expression<'a>>,
    },
    GreaterThanOrEqual {
        lhs: Box<Expression<'a>>,
        rhs: Box<Expression<'a>>,
    },
    LessThanOrEqual {
        lhs: Box<Expression<'a>>,
        rhs: Box<Expression<'a>>,
    },
    Equals {
        lhs: Box<Expression<'a>>,
        rhs: Box<Expression<'a>>,
    },
    Different {
        lhs: Box<Expression<'a>>,
        rhs: Box<Expression<'a>>,
    },
    And {
        lhs: Box<Expression<'a>>,
        rhs: Box<Expression<'a>>,
    },
    Or {
        lhs: Box<Expression<'a>>,
        rhs: Box<Expression<'a>>,
    },
}

#[derive(Debug, PartialEq)]
pub struct Number;

#[derive(Debug, PartialEq)]
pub struct Variable;

#[derive(Debug, PartialEq)]
pub struct Table;
