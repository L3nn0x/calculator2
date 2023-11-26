use std::{iter::Peekable, str::Chars};

use super::token::{Token, Operator};
use log::trace;

fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\r' || c == '\n'
}

pub(crate) fn tokenize(s: String) -> Vec<Token> {
    let mut res = Vec::new();

    let mut it = s.chars().into_iter().peekable();

    let parse_number = |it: &mut Peekable<Chars<'_>>| {
        let mut number = String::new();

        while let Some(&c) = it.peek() {
            trace!("parsing number {:?}", c);
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
        trace!("parsing {:?}", c);
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