use std::{error::Error, fs};

use lust::parser::{
    ast::build_ast,
    expression::Expression,
    prefix_expression::{Argument, CallSuffix, PExprAction, PrefixExpression, Primary, Selector},
    statement::{Block, FunctionName, If, LocalVariable, Return, Statement, Variable},
    LuaParser, Rule,
};
use pest::Parser;
use pretty_assertions::assert_eq;

#[test]
fn markov() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("./lua/markov_chain_algorithm.lua").unwrap();
    let pairs = LuaParser::parse(Rule::Chunk, &content);
    let Ok(mut pairs) = pairs else {
        let err = pairs.err().unwrap();
        println!("{}", err);
        return Err(err.into());
    };
    let ast = build_ast(&mut pairs);

    // Takes to long to format this, I'll try to rewrite this in another way, because *sigh* this way sucks
    #[rustfmt::skip]
    let expected = Block {
        statements: vec![Statement::FunctionDefinition {
            function_name: FunctionName {
                names: vec!["allwords".into()],
                method: None,
            },
            parameters: None,
            body: Block {
                statements: vec![
                    Statement::LocalVariables {
                        variables: vec![LocalVariable {
                            name: "line".into(),
                            attribute: None,
                        }],
                        expr_list: Some(vec![Expression::PrefixExpression(PrefixExpression {
                            primary: Primary::Name("io".into()),
                            actions: vec![
                                PExprAction::Selector(Selector::Dot("read".into())),
                                PExprAction::Call(CallSuffix::Simple(Argument::List(vec![]))),
                            ],
                        })]),
                    },
                    Statement::LocalVariables {
                        variables: vec![LocalVariable {
                            name: "pos".into(),
                            attribute: None,
                        }],
                        expr_list: Some(vec![Expression::Integer(1)]),
                    },
                ],
                return_statement: Some(Return(Some(vec![Expression::Lambda {
                    parameters: None,
                    body: Block {
                        statements: vec![Statement::While {
                            condition: Expression::PrefixExpression(PrefixExpression {
                                primary: Primary::Name("line".into()),
                                actions: vec![],
                            }),
                            block: Block {
                                statements: vec![
                                    Statement::LocalVariables {
                                        variables: vec![
                                            LocalVariable {
                                                name: "s".into(),
                                                attribute: None,
                                            },
                                            LocalVariable {
                                                name: "e".into(),
                                                attribute: None,
                                            },
                                        ],
                                        expr_list: Some(vec![Expression::PrefixExpression(
                                            PrefixExpression {
                                                primary: Primary::Name("string".into()),
                                                actions: vec![
                                                    PExprAction::Selector(Selector::Dot(
                                                        "find".into(),
                                                    )),
                                                    PExprAction::Call(CallSuffix::Simple(
                                                        Argument::List(vec![
                                                            Expression::PrefixExpression(
                                                                PrefixExpression {
                                                                    primary: Primary::Name(
                                                                        "line".into(),
                                                                    ),
                                                                    actions: vec![],
                                                                },
                                                            ),
                                                            Expression::String("%w+".into()),
                                                            Expression::PrefixExpression(
                                                                PrefixExpression {
                                                                    primary: Primary::Name(
                                                                        "pos".into(),
                                                                    ),
                                                                    actions: vec![],
                                                                },
                                                            ),
                                                        ]),
                                                    )),
                                                ],
                                            },
                                        )]),
                                    },
                                    Statement::If {
                                        ifs: vec![If {
                                            condition: Expression::PrefixExpression(
                                                PrefixExpression {
                                                    primary: Primary::Name("s".into()),
                                                    actions: vec![],
                                                },
                                            ),
                                            block: Block {
                                                statements: vec![Statement::Assignment {
                                                    variable_list: vec![Variable::Name(
                                                        "pos".into(),
                                                    )],
                                                    expr_list: vec![Expression::Addition {
                                                        lhs: Box::new(
                                                            Expression::PrefixExpression(
                                                                PrefixExpression {
                                                                    primary: Primary::Name(
                                                                        "e".into(),
                                                                    ),
                                                                    actions: vec![],
                                                                },
                                                            ),
                                                        ),
                                                        rhs: Box::new(Expression::Integer(1)),
                                                    }],
                                                }],
                                                return_statement: Some(Return(Some(vec![
                                                    Expression::PrefixExpression(
                                                        PrefixExpression {
                                                            primary: Primary::Name("string".into()),
                                                            actions: vec![
                                                                PExprAction::Selector(Selector::Dot("sub".into())),
                                                                PExprAction::Call(CallSuffix::Simple(Argument::List(vec![
                                                                    Expression::PrefixExpression(PrefixExpression { primary: Primary::Name("line".into()), actions: vec![] }),
                                                                    Expression::PrefixExpression(PrefixExpression { primary: Primary::Name("s".into()), actions: vec![] }),
                                                                    Expression::PrefixExpression(PrefixExpression { primary: Primary::Name("e".into()), actions: vec![] }),
                                                                ])))
                                                            ],
                                                        },
                                                    ),
                                                ]))),
                                            },
                                        }],
                                        r#else: Some(Block {
                                            statements: vec![
                                                Statement::Assignment { variable_list: vec![Variable::Name("line".into())], expr_list: vec![
                                                    Expression::PrefixExpression(PrefixExpression {
                                                        primary: Primary::Name("io".into()),
                                                        actions: vec![
                                                            PExprAction::Selector(Selector::Dot("read".into())),
                                                            PExprAction::Call(CallSuffix::Simple(Argument::List(vec![]))),
                                                        ]
                                                    })
                                                ]},
                                                Statement::Assignment { variable_list: vec![Variable::Name("pos".into())], expr_list: vec![Expression::Integer(1)] },
                                            ],
                                            return_statement: None,
                                        }),
                                    },
                                ],
                                return_statement: None,
                            },
                        }],
                        return_statement: Some(Return(Some(vec![Expression::Nil]))),
                    },
                }]))),
            },
        }],
        return_statement: None,
    };

    assert_eq!(ast.statements[0], expected.statements[0]);

    println!("{:?}", ast);
    Ok(())
}
