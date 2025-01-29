#[derive(Debug, PartialEq)]
pub struct Position(u32);

impl Position {
    pub fn from_u32(value: u32) -> Self {
        Self(value)
    }

    pub fn to_u32(&self) -> u32 {
        self.0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    start: u32,
    len: u32,
}

impl Span {
    pub fn from_positions(start: Position, end: Position) -> Self {
        Self {
            start: start.to_u32(),
            len: end.to_u32() - start.to_u32(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub value: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralKind {
    Char,
    String,
    Integer,
    Decimal,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Literal {
    pub kind: LiteralKind,
    pub value: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Delimiter {
    Parenthesis,
    Brace,
    Bracket,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Identifier(Identifier),

    Literal(Literal),

    Equals,
    LessThan,
    GreaterThan,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    And,
    Or,

    Dot,
    Comma,
    Semicolon,

    OpenDelimiter(Delimiter),
    CloseDelimiter(Delimiter),

    Eof,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenTree {
    Token(Token),
    Group(Delimiter, Vec<TokenTree>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct TokenStream {
    tokens: Vec<TokenTree>,
}

impl TokenStream {
    pub fn new(tokens: Vec<TokenTree>) -> Self {
        Self { tokens }
    }
}

#[cfg(test)]
mod tests {}
