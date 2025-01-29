pub struct Cursor<'a> {
    chars: std::str::Chars<'a>,
    remaining: u32,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars(),
            remaining: input.len() as u32,
        }
    }

    pub fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    pub fn take_next(&mut self) -> Option<char> {
        self.chars.next()
    }

    pub fn peek_next(&self) -> Option<char> {
        self.chars.clone().next()
    }

    pub fn take_while(&mut self, predicate: impl Fn(char) -> bool) {
        while !self.is_eof() && predicate(self.peek_next().unwrap()) {
            self.take_next();
        }
    }

    pub fn consumed_size(&self) -> u32 {
        self.remaining - self.chars.as_str().len() as u32
    }

    pub fn reset_consumed_size(&mut self) {
        self.remaining = self.chars.as_str().len() as u32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor() {
        let mut cursor = Cursor::new("test");
        assert_eq!(cursor.take_next(), Some('t'));
        assert_eq!(cursor.take_next(), Some('e'));
        assert_eq!(cursor.take_next(), Some('s'));
        assert_eq!(cursor.take_next(), Some('t'));
        assert_eq!(cursor.take_next(), None);
        assert!(cursor.is_eof());
    }

    #[test]
    fn test_cursor_take_while() {
        let mut cursor = Cursor::new("Hello, world!");
        cursor.take_while(|c| c.is_alphabetic());
        assert_eq!(cursor.take_next(), Some(','));
        cursor.take_while(|c| c.is_whitespace());
        cursor.take_while(|c| c.is_alphabetic());
        assert_eq!(cursor.take_next(), Some('!'));
        assert_eq!(cursor.take_next(), None);
        assert!(cursor.is_eof());
    }
}
