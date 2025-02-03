pub mod core;
pub mod literal;
pub mod ty;

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionArgument {
    pub ty: ty::Type,
    pub name: core::Identifier,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub input: Vec<FunctionArgument>,
    pub output: ty::Type,
    // TODO: body
}

#[derive(Debug, PartialEq, Clone)]
pub enum ItemKind {
    Function,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Item {
    pub kind: ItemKind,
}
