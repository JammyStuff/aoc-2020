use std::io::{self, Read};

#[derive(Debug, PartialEq, Eq)]
enum Ops {
    Add,
    Mul,
    LParen,
    RParen,
}

#[derive(Debug, PartialEq, Eq)]
enum Token {
    Op(Ops),
    Value(i64),
}

fn calculate_rpn(expression: &[Token]) -> i64 {
    let mut stack = Vec::new();

    for token in expression {
        match token {
            Token::Value(v) => stack.push(*v),
            Token::Op(Ops::Add) => {
                let lhs = stack.pop().unwrap();
                let rhs = stack.pop().unwrap();
                stack.push(lhs + rhs);
            }
            Token::Op(Ops::Mul) => {
                let lhs = stack.pop().unwrap();
                let rhs = stack.pop().unwrap();
                stack.push(lhs * rhs);
            }
            Token::Op(op) => panic!("Unexpected operator: {:?}", op),
        }
    }

    assert_eq!(stack.len(), 1);
    return stack.pop().unwrap();
}

fn parse_advanced_expression(expression_str: &str) -> Vec<Token> {
    let tokens = tokenize_expression(expression_str);
    let mut expression = Vec::new();
    let mut stack = Vec::new();

    for token in tokens {
        match token {
            Token::Value(_) => expression.push(token),
            Token::Op(Ops::Add) | Token::Op(Ops::LParen) => stack.push(token),
            Token::Op(Ops::Mul) => {
                while let Some(Token::Op(Ops::Add)) = stack.last() {
                    let op = stack.pop().unwrap();
                    expression.push(op);
                }
                stack.push(token);
            },
            Token::Op(Ops::RParen) => {
                while let Some(op) = stack.pop() {
                    if op == Token::Op(Ops::LParen) {
                        break;
                    }
                    expression.push(op);
                }
            }
        }
    }

    while let Some(op) = stack.pop() {
        expression.push(op);
    }

    return expression;
}

fn parse_expression(expression_str: &str) -> Vec<Token> {
    let mut tokens = tokenize_expression(expression_str);
    tokens.reverse();
    let mut expression = Vec::new();
    let mut stack = Vec::new();

    for token in tokens {
        match token {
            Token::Value(_) => expression.push(token),
            Token::Op(Ops::Add) | Token::Op(Ops::Mul) | Token::Op(Ops::RParen) => stack.push(token),
            Token::Op(Ops::LParen) => {
                while let Some(op) = stack.pop() {
                    if op == Token::Op(Ops::RParen) {
                        break;
                    }
                    expression.push(op);
                }
            },
        }
    }

    while let Some(op) = stack.pop() {
        expression.push(op);
    }

    return expression;
}

fn parse_input(input: &str, f: fn(&str) -> Vec<Token>) -> Vec<Vec<Token>> {
    input.lines().map(|l| f(l)).collect()
}

fn tokenize_expression(expression_str: &str) -> Vec<Token> {
    let mut expression = Vec::new();
    let mut number = None;

    for c in expression_str.chars() {
        if let Some(digit) = c.to_digit(10) {
            number = if let Some(value) = number {
                Some((value * 10) + digit)
            } else {
                Some(digit)
            };
            continue;
        }

        if let Some(value) = number {
            expression.push(Token::Value(value as i64));
            number = None;
        }

        match c {
            '+' => expression.push(Token::Op(Ops::Add)),
            '*' => expression.push(Token::Op(Ops::Mul)),
            '(' => expression.push(Token::Op(Ops::LParen)),
            ')' => expression.push(Token::Op(Ops::RParen)),
            ' ' => continue,
            _ => panic!("Unexpected input char: {:?}", c),
        }
    }

    if let Some(value) = number {
        expression.push(Token::Value(value as i64));
    }

    return expression;
}

fn part1(expressions: &[Vec<Token>]) {
    let sum: i64 = expressions.iter().map(|e| calculate_rpn(e)).sum();
    println!("Sum for part 1: {}", sum);
}

fn part2(expressions: &[Vec<Token>]) {
    let sum: i64 = expressions.iter().map(|e| calculate_rpn(e)).sum();
    println!("Sum for part 2: {}", sum);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let expressions = parse_input(&input, parse_expression);
    let advanced_expressions = parse_input(&input, parse_advanced_expression);

    part1(&expressions);
    part2(&advanced_expressions);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_rpn() {
        let expression = vec![
            Token::Value(2),
            Token::Value(4),
            Token::Value(2),
            Token::Value(6),
            Token::Value(6),
            Token::Value(8),
            Token::Value(9),
            Token::Value(6),
            Token::Op(Ops::Add),
            Token::Op(Ops::Mul),
            Token::Op(Ops::Add),
            Token::Value(9),
            Token::Value(4),
            Token::Value(2),
            Token::Op(Ops::Add),
            Token::Op(Ops::Mul),
            Token::Op(Ops::Mul),
            Token::Op(Ops::Add),
            Token::Op(Ops::Add),
            Token::Op(Ops::Add),
            Token::Op(Ops::Mul),
        ];
        assert_eq!(calculate_rpn(&expression), 13632);
    }

    #[test]
    fn test_tokenize_expression() {
        let expression_str = "2 * 3 + (4 * 5)";
        let expected = vec![
            Token::Value(2),
            Token::Op(Ops::Mul),
            Token::Value(3),
            Token::Op(Ops::Add),
            Token::Op(Ops::LParen),
            Token::Value(4),
            Token::Op(Ops::Mul),
            Token::Value(5),
            Token::Op(Ops::RParen),
        ];
        let actual = tokenize_expression(expression_str);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_expression() {
        let expression_str = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let expected = vec![
            Token::Value(2),
            Token::Value(4),
            Token::Value(2),
            Token::Value(6),
            Token::Value(6),
            Token::Value(8),
            Token::Value(9),
            Token::Value(6),
            Token::Op(Ops::Add),
            Token::Op(Ops::Mul),
            Token::Op(Ops::Add),
            Token::Value(9),
            Token::Value(4),
            Token::Value(2),
            Token::Op(Ops::Add),
            Token::Op(Ops::Mul),
            Token::Op(Ops::Mul),
            Token::Op(Ops::Add),
            Token::Op(Ops::Add),
            Token::Op(Ops::Add),
            Token::Op(Ops::Mul),
        ];
        let actual = parse_expression(expression_str);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_advanced_expression() {
        let expression_str = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let expected = vec![
            Token::Value(2),
            Token::Value(4),
            Token::Op(Ops::Add),
            Token::Value(9),
            Token::Op(Ops::Mul),
            Token::Value(6),
            Token::Value(9),
            Token::Op(Ops::Add),
            Token::Value(8),
            Token::Value(6),
            Token::Op(Ops::Add),
            Token::Op(Ops::Mul),
            Token::Value(6),
            Token::Op(Ops::Add),
            Token::Op(Ops::Mul),
            Token::Value(2),
            Token::Value(4),
            Token::Op(Ops::Add),
            Token::Op(Ops::Add),
            Token::Value(2),
            Token::Op(Ops::Mul),
        ];
        let actual = parse_advanced_expression(expression_str);
        assert_eq!(actual, expected);
    }
}
