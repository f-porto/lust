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
        name: Name<'a>,
    },
    Break,
    Goto {
        name: Name<'a>,
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
        alternative: Option<Box<Statement<'a>>>,
    },
    ElseIf {
        condition: Expression<'a>,
        consequence: Block<'a>,
        alternative: Option<Box<Statement<'a>>>,
    },
    Else {
        block: Block<'a>,
    },
    NumericFor {
        name: Name<'a>,
        start: Expression<'a>,
        limit: Expression<'a>,
        step: Option<Expression<'a>>,
        block: Block<'a>,
    },
    GenericFor {
        names: NameList<'a>,
        exprs: Vec<Expression<'a>>,
        block: Block<'a>,
    },
    FunctionDecl {
        name: FunctionName<'a>,
        parameters: ParameterList<'a>,
        block: Block<'a>,
    },
    LocalFunctionDecl {
        name: Name<'a>,
        parameters: ParameterList<'a>,
        block: Block<'a>,
    },
    LocalAttrs {
        attrs: AttributeList<'a>,
        expressions: Vec<Expression<'a>>,
    },
    Return {
        exprs: Option<Vec<Expression<'a>>>,
    },
}

#[derive(Debug, PartialEq)]
pub struct Attribute<'a> {
    pub name: Name<'a>,
    pub attr: Option<Name<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct AttributeList<'a> {
    pub attributes: Vec<Attribute<'a>>,
}

impl<'a> AttributeList<'a> {
    pub fn push(&mut self, attribute: Attribute<'a>) {
        self.attributes.push(attribute);
    }
}

#[derive(Debug, PartialEq)]
pub struct FunctionName<'a> {
    pub name: Name<'a>,
}

#[derive(Debug, PartialEq)]
pub struct ParameterList<'a> {
    pub parameters: NameList<'a>,
    pub var_args: Option<Expression<'a>>,
}

impl<'a> ParameterList<'a> {
    pub fn push(&mut self, parameter: Name<'a>) {
        self.parameters.push(parameter);
    }
}

#[derive(Debug, PartialEq)]
pub struct Name<'a> {
    pub name: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct NameList<'a> {
    pub names: Vec<Name<'a>>,
}

impl<'a> NameList<'a> {
    pub fn push(&mut self, name: Name<'a>) {
        self.names.push(name);
    }
}

#[derive(Debug, PartialEq)]
pub struct Block<'a> {
    pub statements: Vec<Statement<'a>>,
    pub return_statement: Option<Box<Statement<'a>>>,
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
    Variable(Variable<'a>),
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

// TODO: make the table
#[derive(Debug, PartialEq)]
pub struct Number;

#[derive(Debug, PartialEq)]
pub struct Variable<'a> {
    identifier: &'a str,
}

impl<'a> Variable<'a> {
    pub fn new(identifier: &'a str) -> Variable<'a> {
        Variable { identifier }
    }
}

// TODO: make the table
#[derive(Debug, PartialEq)]
pub struct Table;
