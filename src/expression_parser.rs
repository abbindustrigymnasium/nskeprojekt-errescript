use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Operand(i32),
    Plus,
    Minus,
    OpenParen,
    CloseParen,
}

pub fn eval(input: &str) -> i32 {
    let mut tokens = tokenize(input).into_iter().peekable();
    parse_expression(&mut tokens)
}

fn tokenize(input: &str) -> Vec<Token> {
    input
        .split_whitespace()
        .map(|s| match s {
            "+" => Token::Plus,
            "-" => Token::Minus,
            "(" => Token::OpenParen,
            ")" => Token::CloseParen,
            n => match n.parse() {
                Err(e) => {
                    eprintln!("failed to parse number {}: {}", n, e);
                    panic!();
                }
                Ok(num) => Token::Operand(num),
            },
        })
        .collect()
}

fn parse_expression(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> i32 {
    let mut left = parse_primary(tokens);

    while let Some(Token::Plus) | Some(Token::Minus) = tokens.peek() {
        let operator: Option<Token> = tokens.next();
        let right = parse_primary(tokens);
        left = match operator {
            Some(Token::Plus) => left + right,
            Some(Token::Minus) => left - right,
            other => panic!("expected operator, got {:?}", other),
        }
    }

    return left;
}

fn parse_primary(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> i32 {
    match tokens.peek() {
        Some(Token::OpenParen) => {
            tokens.next();
            let val = parse_expression(tokens);
            tokens.next();
            val
        }
        _ => parse_operand(tokens),
    }
}

fn parse_operand(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> i32 {
    match tokens.next() {
        Some(Token::Operand(n)) => n,
        other => panic!("expected number, got {:?}", other),
    }
}
