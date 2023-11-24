use std::cmp::Ordering;
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
        let tokens = shutting_yard(tokens);
        let tokens = match tokens {
            Ok(tokens) => tokens,
            Err(e) => {
                println!("{}", e);
                continue
            }
        };
        let result = evaluate(tokens);
        if let Some(result) = result {
            println!("{result}");
        }
    }
}

fn shutting_yard(tokens: Vec<Token>) -> Result<Vec<Token>, String> {
    let mut output = Vec::new();
    let mut stack = Vec::new();

    for tok in tokens {
        match tok {
            Token::Float(_) | Token::Integer(_) => output.push(tok),
            Token::Operator(o1) => {
                while let Some(op) = stack.last() {
                    let o2 = match op {
                        Token::Operator(o2) => o2,
                        Token::LeftParenthesis => break,
                        _ => unreachable!("Only operators here")
                    };
                    if *o2 > o1 || *o2 == o1 {
                        output.push(*op);
                        stack.pop();
                    } else {
                        break
                    }
                }
                stack.push(Token::Operator(o1));
            },
            Token::LeftParenthesis => {
                stack.push(Token::LeftParenthesis);
            },
            Token::RightParenthesis => {
                let mut found = false;
                while let Some(op) = stack.pop() {
                    if op != Token::LeftParenthesis {
                        output.push(op);
                    } else {
                        found = true;
                        break
                    }
                }
                if !found {
                    return Err("Mismatched parenthesis".to_string());
                }
            }
        }
    }

    while let Some(op) = stack.pop() {
        if op == Token::LeftParenthesis || op == Token::RightParenthesis {
            return Err("Mismatched parenthesis".to_string());
        }
        output.push(op);
    }

    Ok(output)
}

fn evaluate(tokens: Vec<Token>) -> Option<f64> {
    let mut stack = Vec::new();
    for tok in tokens {
        match tok {
            Token::Float(f) => stack.push(f),
            Token::Integer(n) => stack.push(n as f64),
            Token::Operator(p) => {
                let res = p.evaluate(&mut stack)?;
                stack.push(res);
            },
            _ => unreachable!("No parenthesis should be there")
        }
    }
    stack.pop()
}

#[derive(Debug, PartialEq, Eq, Ord, Clone, Copy)]
enum Operator {
    Plus,
    Minus,
    Div,
    Mul
}

impl PartialOrd for Operator {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.to_prio().cmp(&other.to_prio()))
    }
}

fn apply<F: FnOnce(f64, f64)->f64>(stack: &mut Vec<f64>, f: F) -> f64 {
    let b = stack.pop().unwrap();
    let a = stack.pop().unwrap();
    f(a, b)
}

impl Operator {
    fn to_prio(&self) -> i32 {
        match *self {
            Operator::Mul | Operator::Div => 2,
            _ => 1
        }
    }

    fn from(value: char) -> Option<Operator> {
        match value {
            '+' => Some(Operator::Plus),
            '-' => Some(Operator::Minus),
            '/' => Some(Operator::Div),
            '*' => Some(Operator::Mul),
            _ => None
        }
    }

    fn evaluate(&self, stack: &mut Vec<f64>) -> Option<f64> {
        if stack.len() < 2 {
            return None;
        }
        match *self {
            Operator::Plus => {
                Some(apply(stack, |a,b| a+b))
            },
            Operator::Minus => {
                Some(apply(stack, |a,b|a-b))
            },
            Operator::Mul => {
                Some(apply(stack,|a,b|a*b))
            },
            Operator::Div => {
                Some(apply(stack,|a,b|a/b))
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Token {
    Integer(i64),
    Float(f64),
    Operator(Operator),
    LeftParenthesis,
    RightParenthesis
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
            if c == '(' || c == ')' || Operator::from(c).is_some() || is_whitespace(c) && !c.is_ascii_hexdigit() && c != '.' && c != 'x' && c != 'X' {
                break;
            }
            number.push(c);
            it.next();
        }

        number
    };

    let mut minus = false;
    let mut previous_char = '\0';
    while let Some(&c) = it.peek() {
        if is_whitespace(c) {
            if minus {
                minus = false;
                res.push(Token::Operator(Operator::Minus));
            }
            it.next();
            continue;
        }
        if c == '(' {
            res.push(Token::LeftParenthesis);
            it.next();
        } else if c == ')' {
            res.push(Token::RightParenthesis);
            it.next();
        } else if let Some(op) = Operator::from(c) {
            if op == Operator::Minus && (is_whitespace(previous_char) && !minus) {
                minus = true;
            } else {
                minus = false;
                res.push(Token::Operator(op));
            }
            it.next();
        } else {
            let number = parse_number(&mut it);
            let m = if minus { -1 } else { 1 };
            minus = false;
            if number.starts_with("0x") || number.starts_with("0X") {
                if let Ok(n) = i64::from_str_radix(&number[2..], 16) {
                    res.push(Token::Integer(m * n));
                }
            } else if number.contains('.') {
                if let Ok(n) = number.parse::<f64>() {
                    res.push(Token::Float(m as f64 * n));
                }
            } else {
                if let Ok(n) = number.parse::<i64>() {
                    res.push(Token::Integer(m * n));
                }
            }
        }
        previous_char = c;
    }

    res
}