fn run(code: &str) -> i32 {
    let mut stack = Vec::new();
    for token in code.split_whitespace() {
        if let Ok(num) = token.parse::<i32>() {
            stack.push(num);
        } else {
            match token {
                "+" => {
                    let (a, b) = (stack.pop().unwrap(), stack.pop().unwrap());
                    stack.push(b + a);
                }
                "-" => {
                    let (a, b) = (stack.pop().unwrap(), stack.pop().unwrap());
                    stack.push(b - a);
                }
                "*" => {
                    let (a, b) = (stack.pop().unwrap(), stack.pop().unwrap());
                    stack.push(b * a);
                }
                "/" => {
                    let (a, b) = (stack.pop().unwrap(), stack.pop().unwrap());
                    stack.push(b / a);
                }
                _ => todo!("unknown operator"),
            }
        }
    }
    stack.pop().unwrap()
}

fn main() {
    let code = "1 2 +";
    let result = run(code);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(run("1 2 +"), 3);
        assert_eq!(run("3 4 -"), -1);
        assert_eq!(run("5 6 *"), 30);
        assert_eq!(run("8 2 /"), 4);
        assert_eq!(run("1 2 + 3 4 - *"), -3);
        assert_eq!(run("42 36 + 22 +"), 100);
    }
}
