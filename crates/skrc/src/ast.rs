pub mod core;
pub mod literal;
pub mod ty;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },
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
    pub ty: ty::Type,
    pub name: core::Identifier,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub input: Vec<FunctionArgument>,
    pub output: ty::Type,
    pub body: Block,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Item {
    Function(Function),
}
