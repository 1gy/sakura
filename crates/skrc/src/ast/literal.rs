#[derive(Debug, PartialEq, Clone)]
pub enum LiteralKind {
    Integer,
    Decimal,
    String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Literal {
    pub kind: LiteralKind,
    pub symbol: super::core::Symbol,
}
