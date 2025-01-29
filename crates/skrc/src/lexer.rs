pub mod cursor;
pub mod scanner;

type Position = u32; // TODO: refactor to newtype

pub struct Lexer<'code> {
    code: &'code str,
    position: Position,
    scanner: scanner::Scanner<'code>,
}

impl Lexer<'_> {
    fn identifier(&self, start: Position) -> super::token::TokenKind {
        let value = &self.code[start as usize..self.position as usize];
        super::token::TokenKind::Identifier(super::token::Identifier {
            value: value.to_string(),
        })
    }

    fn literal(&self, start: Position, kind: scanner::ScannedTokenKind) -> super::token::TokenKind {
        use super::token::{Literal, LiteralKind, TokenKind};
        use scanner::ScannedTokenKind;

        let value = &self.code[start as usize..self.position as usize];
        let literal_kind = match kind {
            ScannedTokenKind::CharLiteral => LiteralKind::Char,
            ScannedTokenKind::StringLiteral => LiteralKind::String,
            ScannedTokenKind::IntegerLiteral => LiteralKind::Integer,
            ScannedTokenKind::DecimalLiteral => LiteralKind::Decimal,
            _ => unreachable!(),
        };
        TokenKind::Literal(Literal {
            kind: literal_kind,
            value: value.to_string(),
        })
    }

    fn next_token(&mut self) -> super::token::Token {
        use super::token::{Delimiter, Position, Span, Token, TokenKind};
        use scanner::ScannedTokenKind;

        loop {
            let scanned_token = self.scanner.next_token();
            let start = self.position;
            self.position += scanned_token.len;
            let token_kind = match scanned_token.kind {
                ScannedTokenKind::Whitespace => continue, // skip
                ScannedTokenKind::Identifier => self.identifier(start),
                ScannedTokenKind::CharLiteral
                | ScannedTokenKind::StringLiteral
                | ScannedTokenKind::IntegerLiteral
                | ScannedTokenKind::DecimalLiteral => self.literal(start, scanned_token.kind),
                ScannedTokenKind::Equals => TokenKind::Equals,
                ScannedTokenKind::LessThan => TokenKind::LessThan,
                ScannedTokenKind::GreaterThan => TokenKind::GreaterThan,
                ScannedTokenKind::Plus => TokenKind::Plus,
                ScannedTokenKind::Minus => TokenKind::Minus,
                ScannedTokenKind::Star => TokenKind::Star,
                ScannedTokenKind::Slash => TokenKind::Slash,
                ScannedTokenKind::Percent => TokenKind::Percent,
                ScannedTokenKind::Caret => TokenKind::Caret,
                ScannedTokenKind::And => TokenKind::And,
                ScannedTokenKind::Or => TokenKind::Or,
                ScannedTokenKind::Dot => TokenKind::Dot,
                ScannedTokenKind::Comma => TokenKind::Comma,
                ScannedTokenKind::Semicolon => TokenKind::Semicolon,
                ScannedTokenKind::OpenParenthesis => {
                    TokenKind::OpenDelimiter(Delimiter::Parenthesis)
                }
                ScannedTokenKind::CloseParenthesis => {
                    TokenKind::CloseDelimiter(Delimiter::Parenthesis)
                }
                ScannedTokenKind::OpenBrace => TokenKind::OpenDelimiter(Delimiter::Brace),
                ScannedTokenKind::CloseBrace => TokenKind::CloseDelimiter(Delimiter::Brace),
                ScannedTokenKind::OpenBracket => TokenKind::OpenDelimiter(Delimiter::Bracket),
                ScannedTokenKind::CloseBracket => TokenKind::CloseDelimiter(Delimiter::Bracket),
                ScannedTokenKind::Eof => TokenKind::Eof,
                ScannedTokenKind::Unknown => unimplemented!("Unknown token"),
            };
            let span =
                Span::from_positions(Position::from_u32(start), Position::from_u32(self.position));
            return Token::new(token_kind, span);
        }
    }

    pub fn lex_token_trees(&mut self) -> super::token::TokenStream {
        use super::token::{Delimiter, TokenKind, TokenStream, TokenTree};

        enum StackType {
            Parenthesis,
            Brace,
            Bracket,
            Root,
        }

        let mut stack: Vec<(StackType, Vec<TokenTree>)> = vec![(StackType::Root, Vec::new())];

        // TODO: remove unwrap, panic

        loop {
            let token = self.next_token();
            match token.kind.clone() {
                TokenKind::OpenDelimiter(delimiter) => {
                    stack.push((
                        match delimiter {
                            Delimiter::Parenthesis => StackType::Parenthesis,
                            Delimiter::Brace => StackType::Brace,
                            Delimiter::Bracket => StackType::Bracket,
                        },
                        Vec::new(),
                    ));
                }
                TokenKind::CloseDelimiter(delimiter) => {
                    let (stack_type, tokens) = stack.pop().unwrap();
                    match (stack_type, delimiter.clone()) {
                        (StackType::Parenthesis, Delimiter::Parenthesis)
                        | (StackType::Brace, Delimiter::Brace)
                        | (StackType::Bracket, Delimiter::Bracket) => {
                            let (_parent_stack_type, parent_tokens) = stack.last_mut().unwrap();
                            parent_tokens.push(TokenTree::Group(delimiter.clone(), tokens));
                        }
                        _ => panic!("Mismatched delimiters"),
                    }
                }
                TokenKind::Eof => {
                    if stack.len() != 1 {
                        panic!("Mismatched delimiters");
                    }
                    let (_stack_type, tokens) = stack.pop().unwrap();
                    return TokenStream::new(tokens);
                }
                _ => {
                    let (_stack_type, tokens) = stack.last_mut().unwrap();
                    tokens.push(TokenTree::Token(token));
                }
            }
        }
    }
}

pub fn lex_token_trees(code: &str) -> super::token::TokenStream {
    let mut lexer = Lexer {
        code,
        position: 0,
        scanner: scanner::Scanner::new(code),
    };
    lexer.lex_token_trees()
}

#[cfg(test)]
mod tests {
    use crate::token::{
        Delimiter, Identifier, Literal, LiteralKind, Position, Span, Token, TokenKind, TokenStream,
        TokenTree,
    };

    use super::*;

    #[test]
    fn test_lexer_next_token() {
        let code = "let x = 42;";
        let mut lexer = Lexer {
            code,
            position: 0,
            scanner: scanner::Scanner::new(code),
        };
        let token = lexer.next_token();
        assert_eq!(
            token,
            Token::new(
                TokenKind::Identifier(Identifier {
                    value: "let".to_string()
                }),
                Span::from_positions(Position::from_u32(0), Position::from_u32(3))
            )
        );
        let token = lexer.next_token();
        assert_eq!(
            token,
            Token::new(
                TokenKind::Identifier(Identifier {
                    value: "x".to_string()
                }),
                Span::from_positions(Position::from_u32(4), Position::from_u32(5))
            )
        );
        let token = lexer.next_token();
        assert_eq!(
            token,
            Token::new(
                TokenKind::Equals,
                Span::from_positions(Position::from_u32(6), Position::from_u32(7))
            )
        );
        let token = lexer.next_token();
        assert_eq!(
            token,
            Token::new(
                TokenKind::Literal(Literal {
                    kind: LiteralKind::Integer,
                    value: "42".to_string()
                }),
                Span::from_positions(Position::from_u32(8), Position::from_u32(10))
            )
        );
        let token = lexer.next_token();
        assert_eq!(
            token,
            Token::new(
                TokenKind::Semicolon,
                Span::from_positions(Position::from_u32(10), Position::from_u32(11))
            )
        );
        let token = lexer.next_token();
        assert_eq!(
            token,
            Token::new(
                TokenKind::Eof,
                Span::from_positions(Position::from_u32(11), Position::from_u32(11))
            )
        );
    }

    #[test]
    fn test_lex_token_trees() {
        let code = "let x = 42;";
        let token_stream = lex_token_trees(code);
        assert_eq!(
            token_stream,
            TokenStream::new(vec![
                TokenTree::Token(Token::new(
                    TokenKind::Identifier(Identifier {
                        value: "let".to_string()
                    }),
                    Span::from_positions(Position::from_u32(0), Position::from_u32(3))
                )),
                TokenTree::Token(Token::new(
                    TokenKind::Identifier(Identifier {
                        value: "x".to_string()
                    }),
                    Span::from_positions(Position::from_u32(4), Position::from_u32(5))
                )),
                TokenTree::Token(Token::new(
                    TokenKind::Equals,
                    Span::from_positions(Position::from_u32(6), Position::from_u32(7))
                )),
                TokenTree::Token(Token::new(
                    TokenKind::Literal(Literal {
                        kind: LiteralKind::Integer,
                        value: "42".to_string()
                    }),
                    Span::from_positions(Position::from_u32(8), Position::from_u32(10))
                )),
                TokenTree::Token(Token::new(
                    TokenKind::Semicolon,
                    Span::from_positions(Position::from_u32(10), Position::from_u32(11))
                )),
            ])
        );
    }

    #[test]
    fn test_lex_token_trees_nested() {
        let code = "let x = (40 + 2);";
        let token_stream = lex_token_trees(code);
        assert_eq!(
            token_stream,
            TokenStream::new(vec![
                TokenTree::Token(Token::new(
                    TokenKind::Identifier(Identifier {
                        value: "let".to_string()
                    }),
                    Span::from_positions(Position::from_u32(0), Position::from_u32(3))
                )),
                TokenTree::Token(Token::new(
                    TokenKind::Identifier(Identifier {
                        value: "x".to_string()
                    }),
                    Span::from_positions(Position::from_u32(4), Position::from_u32(5))
                )),
                TokenTree::Token(Token::new(
                    TokenKind::Equals,
                    Span::from_positions(Position::from_u32(6), Position::from_u32(7))
                )),
                TokenTree::Group(
                    Delimiter::Parenthesis,
                    vec![
                        TokenTree::Token(Token::new(
                            TokenKind::Literal(Literal {
                                kind: LiteralKind::Integer,
                                value: "40".to_string()
                            }),
                            Span::from_positions(Position::from_u32(9), Position::from_u32(11))
                        )),
                        TokenTree::Token(Token::new(
                            TokenKind::Plus,
                            Span::from_positions(Position::from_u32(12), Position::from_u32(13))
                        )),
                        TokenTree::Token(Token::new(
                            TokenKind::Literal(Literal {
                                kind: LiteralKind::Integer,
                                value: "2".to_string()
                            }),
                            Span::from_positions(Position::from_u32(14), Position::from_u32(15))
                        )),
                    ]
                ),
                TokenTree::Token(Token::new(
                    TokenKind::Semicolon,
                    Span::from_positions(Position::from_u32(16), Position::from_u32(17))
                )),
            ])
        );
    }

    #[test]
    fn test_lex_token_trees_nested2() {
        let code = "{([])[{}]}()[]";
        let token_stream = lex_token_trees(code);
        assert_eq!(
            token_stream,
            TokenStream::new(vec![
                TokenTree::Group(
                    Delimiter::Brace,
                    vec![
                        TokenTree::Group(
                            Delimiter::Parenthesis,
                            vec![TokenTree::Group(Delimiter::Bracket, vec![]),]
                        ),
                        TokenTree::Group(
                            Delimiter::Bracket,
                            vec![TokenTree::Group(Delimiter::Brace, vec![]),]
                        ),
                    ]
                ),
                TokenTree::Group(Delimiter::Parenthesis, vec![]),
                TokenTree::Group(Delimiter::Bracket, vec![]),
            ])
        );
    }
}
