use super::cursor::Cursor;

fn is_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n' | '\r')
}

fn is_valid_identifier_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_valid_identifier_continue(c: char) -> bool {
    is_valid_identifier_start(c) || c.is_ascii_digit()
}

#[derive(Debug, PartialEq)]
pub enum ScannedTokenKind {
    Whitespace,

    Identifier,

    CharLiteral,
    StringLiteral,
    IntegerLiteral,
    DecimalLiteral,

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

    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,

    Eof,

    Unknown,
}

#[derive(Debug, PartialEq)]
pub struct ScannedToken {
    pub kind: ScannedTokenKind,
    pub len: u32,
}

impl ScannedToken {
    pub fn new(kind: ScannedTokenKind, len: u32) -> Self {
        Self { kind, len }
    }
}

pub struct Scanner<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            cursor: Cursor::new(input),
        }
    }

    fn whitespace(&mut self) -> ScannedTokenKind {
        self.cursor.take_while(is_whitespace);
        ScannedTokenKind::Whitespace
    }

    fn identifier(&mut self) -> ScannedTokenKind {
        self.cursor.take_while(is_valid_identifier_continue);
        ScannedTokenKind::Identifier
    }

    fn numeric_literal(&mut self) -> ScannedTokenKind {
        self.cursor.take_while(|c| c.is_ascii_digit());
        match self.cursor.peek_next() {
            Some('.') => {
                self.cursor.take_next();
                self.cursor.take_while(|c| c.is_ascii_digit());
                ScannedTokenKind::DecimalLiteral
            }
            _ => ScannedTokenKind::IntegerLiteral,
        }
    }

    fn char_literal(&mut self) -> ScannedTokenKind {
        self.cursor.take_while(|c| c != '\'');
        self.cursor.take_next();
        ScannedTokenKind::CharLiteral
    }

    fn string_literal(&mut self) -> ScannedTokenKind {
        self.cursor.take_while(|c| c != '"');
        self.cursor.take_next();
        ScannedTokenKind::StringLiteral
    }

    pub fn next_token(&mut self) -> ScannedToken {
        let c = match self.cursor.take_next() {
            Some(c) => c,
            None => return ScannedToken::new(ScannedTokenKind::Eof, 0),
        };
        let token_kind = match c {
            c if is_whitespace(c) => self.whitespace(),
            c if is_valid_identifier_start(c) => self.identifier(),
            '\'' => self.char_literal(),
            '"' => self.string_literal(),
            '0'..='9' => self.numeric_literal(),
            '=' => ScannedTokenKind::Equals,
            '<' => ScannedTokenKind::LessThan,
            '>' => ScannedTokenKind::GreaterThan,
            '+' => ScannedTokenKind::Plus,
            '-' => ScannedTokenKind::Minus,
            '*' => ScannedTokenKind::Star,
            '/' => ScannedTokenKind::Slash,
            '%' => ScannedTokenKind::Percent,
            '^' => ScannedTokenKind::Caret,
            '&' => ScannedTokenKind::And,
            '|' => ScannedTokenKind::Or,
            '.' => ScannedTokenKind::Dot,
            ',' => ScannedTokenKind::Comma,
            ';' => ScannedTokenKind::Semicolon,
            '(' => ScannedTokenKind::OpenParenthesis,
            ')' => ScannedTokenKind::CloseParenthesis,
            '{' => ScannedTokenKind::OpenBrace,
            '}' => ScannedTokenKind::CloseBrace,
            '[' => ScannedTokenKind::OpenBracket,
            ']' => ScannedTokenKind::CloseBracket,
            _ => ScannedTokenKind::Unknown,
        };
        let token = ScannedToken::new(token_kind, self.cursor.consumed_size());
        self.cursor.reset_consumed_size();
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_whitespace() {
        assert!(is_whitespace(' '));
        assert!(is_whitespace('\t'));
        assert!(is_whitespace('\n'));
        assert!(is_whitespace('\r'));
        assert!(!is_whitespace('a'));
    }

    #[test]
    fn test_is_valid_identifier_start() {
        assert!(is_valid_identifier_start('a'));
        assert!(is_valid_identifier_start('A'));
        assert!(is_valid_identifier_start('_'));
        assert!(!is_valid_identifier_start('1'));
    }

    #[test]
    fn test_is_valid_identifier_continue() {
        assert!(is_valid_identifier_continue('a'));
        assert!(is_valid_identifier_continue('A'));
        assert!(is_valid_identifier_continue('_'));
        assert!(is_valid_identifier_continue('1'));
        assert!(!is_valid_identifier_continue('.'));
    }

    #[test]
    fn test_whitespace() {
        let mut scanner = Scanner::new(" \t\n\r");
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::Whitespace, 4)
        );
    }

    #[test]
    fn test_identifier() {
        let mut scanner = Scanner::new("abc_123");
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::Identifier, 7)
        );
    }

    #[test]
    fn test_numeric_literal() {
        let mut scanner = Scanner::new("123 456.789");
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::IntegerLiteral, 3)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::Whitespace, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::DecimalLiteral, 7)
        );
    }

    #[test]
    fn test_char_literal() {
        let mut scanner = Scanner::new("'a' 'b'");
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::CharLiteral, 3)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::Whitespace, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::CharLiteral, 3)
        );
    }

    #[test]
    fn test_string_literal() {
        let mut scanner = Scanner::new("\"abc\" \"def\"");
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::StringLiteral, 5)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::Whitespace, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::StringLiteral, 5)
        );
    }

    #[test]
    fn test_symbols() {
        let mut scanner = Scanner::new("= < > + - * / % ^ & | . , ; ( ) { } [ ]");
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::Equals, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::Whitespace, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::LessThan, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::Whitespace, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::GreaterThan, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::Whitespace, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::Plus, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::Whitespace, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::Minus, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::Whitespace, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::Star, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::Whitespace, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::Slash, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::Whitespace, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::Percent, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::Whitespace, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::Caret, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::Whitespace, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::And, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::Whitespace, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::Or, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::Whitespace, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::Dot, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::Whitespace, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::Comma, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::Whitespace, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::Semicolon, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::Whitespace, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::OpenParenthesis, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::Whitespace, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::CloseParenthesis, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::Whitespace, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::OpenBrace, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::Whitespace, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::CloseBrace, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::Whitespace, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::OpenBracket, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::Whitespace, 1)
        );
        assert_eq!(
            scanner.next_token(),
            ScannedToken::new(ScannedTokenKind::CloseBracket, 1)
        );
    }
}
