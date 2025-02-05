#[skrc_branded::branded]
pub type NodeId = u32;

#[skrc_branded::branded]
pub type Symbol = String;

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub symbol: Symbol,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Path {
    pub segments: Vec<Identifier>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Path(Path),
    // TODO: Array, Slice, ...
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Integer(Symbol),
    Decimal(Symbol),
    String(Symbol),
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    LogicalAnd,
    LogicalOr,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Literal(Literal),
    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },
    CallMethod {
        receiver: Box<Expression>,
        method: Identifier,
        arguments: Vec<Expression>,
    },
    If {
        condition: Box<Expression>,
        then_branch: Box<Block>,
        else_branch: Option<Block>,
    },
    BinaryOperation {
        operator: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Path(Box<Path>),
    Return(Option<Box<Expression>>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Expression(Expression),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionArgument {
    pub ty: Type,
    pub name: Identifier,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub input: Vec<FunctionArgument>,
    pub output: Type,
    pub body: Block,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Item {
    Function(Function),
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    // サンプルコード
    fn fizzbuzz(i: u32) -> String {
        if (i % 3) == 0 && (i % 5) == 0 {
            return "FizzBuzz";
        } else if (i % 3) == 0 {
            return "Fizz";
        } else if (i % 5) == 0 {
            return "Buzz";
        } else {
            return i.to_string();
        }
    }
    */

    #[test]
    fn ast_sample() {
        let _ast = Item::Function(Function {
            input: vec![FunctionArgument {
                ty: Type::Path(Path {
                    segments: vec![Identifier {
                        symbol: "u32".to_string().into(),
                    }],
                }),
                name: Identifier {
                    symbol: "i".to_string().into(),
                },
            }],
            output: Type::Path(Path {
                segments: vec![Identifier {
                    symbol: "String".to_string().into(),
                }],
            }),
            body: Block {
                statements: vec![
                    Statement::Expression(Expression::If {
                        condition: Box::new(Expression::BinaryOperation {
                            operator: BinaryOperator::LogicalAnd,
                            left: Box::new(Expression::BinaryOperation {
                                operator: BinaryOperator::Equal,
                                left: Box::new(Expression::BinaryOperation {
                                    operator: BinaryOperator::Mod,
                                    left: Box::new(Expression::Path(Box::new(Path {
                                        segments: vec![Identifier {
                                            symbol: "i".to_string().into(),
                                        }],
                                    }))),
                                    right: Box::new(Expression::Literal(Literal::Integer(
                                        "3".to_string().into(),
                                    ))),
                                }),
                                right: Box::new(Expression::Literal(Literal::Integer(
                                    "0".to_string().into(),
                                ))),
                            }),
                            right: Box::new(Expression::BinaryOperation {
                                operator: BinaryOperator::Equal,
                                left: Box::new(Expression::BinaryOperation {
                                    operator: BinaryOperator::Mod,
                                    left: Box::new(Expression::Path(Box::new(Path {
                                        segments: vec![Identifier {
                                            symbol: "i".to_string().into(),
                                        }],
                                    }))),
                                    right: Box::new(Expression::Literal(Literal::Integer(
                                        "5".to_string().into(),
                                    ))),
                                }),
                                right: Box::new(Expression::Literal(Literal::Integer(
                                    "0".to_string().into(),
                                ))),
                            }),
                        }),
                        then_branch: Box::new(Block {
                            statements: vec![Statement::Expression(Expression::Return(Some(
                                Box::new(Expression::Literal(Literal::String(
                                    "FizzBuzz".to_string().into(),
                                ))),
                            )))],
                        }),
                        else_branch: Some(Block {
                            statements: vec![Statement::Expression(Expression::If {
                                condition: Box::new(Expression::BinaryOperation {
                                    operator: BinaryOperator::Equal,
                                    left: Box::new(Expression::BinaryOperation {
                                        operator: BinaryOperator::Mod,
                                        left: Box::new(Expression::Path(Box::new(Path {
                                            segments: vec![Identifier {
                                                symbol: "i".to_string().into(),
                                            }],
                                        }))),
                                        right: Box::new(Expression::Literal(Literal::Integer(
                                            "3".to_string().into(),
                                        ))),
                                    }),
                                    right: Box::new(Expression::Literal(Literal::Integer(
                                        "0".to_string().into(),
                                    ))),
                                }),
                                then_branch: Box::new(Block {
                                    statements: vec![Statement::Expression(Expression::Literal(
                                        Literal::String("Fizz".to_string().into()),
                                    ))],
                                }),
                                else_branch: Some(Block {
                                    statements: vec![Statement::Expression(Expression::If {
                                        condition: Box::new(Expression::BinaryOperation {
                                            operator: BinaryOperator::Equal,
                                            left: Box::new(Expression::BinaryOperation {
                                                operator: BinaryOperator::Mod,
                                                left: Box::new(Expression::Path(Box::new(Path {
                                                    segments: vec![Identifier {
                                                        symbol: "i".to_string().into(),
                                                    }],
                                                }))),
                                                right: Box::new(Expression::Literal(
                                                    Literal::Integer("5".to_string().into()),
                                                )),
                                            }),
                                            right: Box::new(Expression::Literal(Literal::Integer(
                                                "0".to_string().into(),
                                            ))),
                                        }),
                                        then_branch: Box::new(Block {
                                            statements: vec![Statement::Expression(
                                                Expression::Return(Some(Box::new(
                                                    Expression::Literal(Literal::String(
                                                        "Buzz".to_string().into(),
                                                    )),
                                                ))),
                                            )],
                                        }),
                                        else_branch: Some(Block {
                                            statements: vec![Statement::Expression(
                                                Expression::CallMethod {
                                                    receiver: Box::new(Expression::Path(Box::new(
                                                        Path {
                                                            segments: vec![Identifier {
                                                                symbol: "i".to_string().into(),
                                                            }],
                                                        },
                                                    ))),
                                                    method: Identifier {
                                                        symbol: "to_string".to_string().into(),
                                                    },
                                                    arguments: vec![],
                                                },
                                            )],
                                        }),
                                    })],
                                }),
                            })],
                        }),
                    }),
                    Statement::Expression(Expression::Return(Some(Box::new(Expression::Path(
                        Box::new(Path {
                            segments: vec![Identifier {
                                symbol: "i".to_string().into(),
                            }],
                        }),
                    ))))),
                ],
            },
        });
    }
}
