use crate::token::Span;

#[derive(Debug, PartialEq, Clone)]
pub struct Call {
    pub function: Box<Expression>,
    pub arguments: Vec<Expression>,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Literal {
    pub value: String,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Let {
    pub name: String,
    pub expression: Box<Expression>,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub struct If {
    pub condition: Box<Expression>,
    pub then: Box<Block>,
    pub else_: Option<Box<Block>>,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Call(Call),
    Literal(Literal),
    Let(Let),
    If(If),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Expression(Expression),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub span: Span,
}

// e.g. i32, f32, bool
#[derive(Debug, PartialEq, Clone)]
pub struct Type {
    pub name: String,
    pub span: Span,
}

// e.g. a: i32
#[derive(Debug, PartialEq, Clone)]
pub struct FunctionParameter {
    pub name: String,
    pub ty: Type,
    pub span: Span,
}

// e.g. fn foo(a: i32, b: i32) -> i32
#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub name: String,
    pub input: Vec<FunctionParameter>,
    pub output: Type,
    pub body: Vec<Block>,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ItemKind {
    Function(Box<Function>),
}
