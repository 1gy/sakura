#[derive(Debug, PartialEq)]
pub struct Parser {}

impl Parser {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let parser = Parser {};
        assert_eq!(parser, Parser {});
    }
}
