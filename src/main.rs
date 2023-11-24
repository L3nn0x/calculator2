use std::iter::Peekable;
use std::str::Chars;
use std::io::{stdin, stdout, Write};


fn main() {
    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut input = String::new();
        if let Ok(n) = stdin().read_line(&mut input) {
            if n == 0 {
                break;
            }
        } else {
            break;
        }
        let tokens = tokenize(input);
        println!("{:?}", tokens);
    }
}

#[derive(Debug)]
enum Operator {
    Plus,
    Minus,
    Div,
    Mul
}

impl Operator {
    fn from(value: char) -> Option<Operator> {
        match value {
            '+' => Some(Operator::Plus),
            '-' => Some(Operator::Minus),
            '/' => Some(Operator::Div),
            '*' => Some(Operator::Mul),
            _ => None
        }
    }
}

#[derive(Debug)]
enum Token {
    Integer(i64),
    Float(f64),
    Operator(Operator)
}

fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\r' || c == '\n'
}

fn tokenize(s: String) -> Vec<Token> {
    let mut res = Vec::new();

    let mut it = s.chars().into_iter().peekable();

    let parse_number = |it: &mut Peekable<Chars<'_>>| {
        let mut number = String::new();

        while let Some(&c) = it.peek() {
            if is_whitespace(c) && !c.is_ascii_hexdigit() && c != '.' && c != 'x' && c != 'X' {
                break;
            }
            number.push(c);
            it.next();
        }

        number
    };

    while let Some(&c) = it.peek() {
        if is_whitespace(c) {
            it.next();
            continue;
        }
        if let Some(op) = Operator::from(c) {
            res.push(Token::Operator(op));
            it.next();
        } else {
            let number = parse_number(&mut it);
            if number.starts_with("0x") || number.starts_with("0X") {
                if let Ok(n) = i64::from_str_radix(&number[2..], 16) {
                    res.push(Token::Integer(n));
                }
            } else if number.contains('.') {
                if let Ok(n) = number.parse::<f64>() {
                    res.push(Token::Float(n));
                }
            } else {
                if let Ok(n) = number.parse::<i64>() {
                    res.push(Token::Integer(n));
                }
            }
        }
    }

    res
}