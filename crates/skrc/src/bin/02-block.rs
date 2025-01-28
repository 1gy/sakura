#[derive(Debug, PartialEq)]
enum Value<'a> {
    Number(i32),
    Operator(&'a str),
    Block(Vec<Value<'a>>),
}

impl Value<'_> {
    fn as_number(&self) -> i32 {
        match self {
            Value::Number(num) => *num,
            _ => panic!("expected number"),
        }
    }
}

fn parse_block<'a, 'code>(input: &'a [&'code str]) -> (Value<'code>, &'a [&'code str]) {
    let mut words = input;
    let mut current_tokens = Vec::new();

    while let Some((&token, rest)) = words.split_first() {
        if token.is_empty() {
            break;
        }
        let mut rest = rest;
        if token == "{" {
            let value;
            (value, rest) = parse_block(rest);
            current_tokens.push(value);
        } else if token == "}" {
            return (Value::Block(current_tokens), rest);
        } else if let Ok(num) = token.parse::<i32>() {
            current_tokens.push(Value::Number(num));
        } else {
            current_tokens.push(Value::Operator(token));
        }
        words = rest;
    }
    (Value::Block(current_tokens), words)
}

fn run(code: &str) -> Vec<Value<'_>> {
    let mut stack = Vec::new();
    let input: Vec<_> = code.split_whitespace().collect();
    let mut tokens = &input[..];
    while let Some((&token, rest)) = tokens.split_first() {
        if token.is_empty() {
            break;
        }
        let mut rest = rest;
        if token == "{" {
            let block;
            (block, rest) = parse_block(rest);
            stack.push(block);
        } else if let Ok(num) = token.parse::<i32>() {
            stack.push(Value::Number(num));
        } else {
            match token {
                "+" => {
                    let (a, b) = (stack.pop().unwrap(), stack.pop().unwrap());
                    stack.push(Value::Number(b.as_number() + a.as_number()));
                }
                "-" => {
                    let (a, b) = (stack.pop().unwrap(), stack.pop().unwrap());
                    stack.push(Value::Number(b.as_number() - a.as_number()));
                }
                "*" => {
                    let (a, b) = (stack.pop().unwrap(), stack.pop().unwrap());
                    stack.push(Value::Number(b.as_number() * a.as_number()));
                }
                "/" => {
                    let (a, b) = (stack.pop().unwrap(), stack.pop().unwrap());
                    stack.push(Value::Number(b.as_number() / a.as_number()));
                }
                _ => todo!("unknown operator"),
            }
        }
        tokens = rest;
    }

    stack
}

fn main() {
    let code = "1 2 + { 3 4 }";
    let result = run(code);
    dbg!(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(run("1 2 +"), vec![Value::Number(3)]);
        assert_eq!(run("3 4 -"), vec![Value::Number(-1)]);
        assert_eq!(run("5 6 *"), vec![Value::Number(30)]);
        assert_eq!(run("8 2 /"), vec![Value::Number(4)]);
        assert_eq!(run("1 2 + 3 4 - *"), vec![Value::Number(-3)]);
        assert_eq!(run("42 36 + 22 +"), vec![Value::Number(100)]);
        assert_eq!(
            run("1 2 + { 3 4 }"),
            vec![
                Value::Number(3),
                Value::Block(vec![Value::Number(3), Value::Number(4)])
            ]
        );
        assert_eq!(
            run("{ { 1 2 3 } }"),
            vec![Value::Block(vec![Value::Block(vec![
                Value::Number(1),
                Value::Number(2),
                Value::Number(3)
            ])])]
        );
    }
}
