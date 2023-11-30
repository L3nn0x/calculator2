use super::token::{Token, Operator};
use super::tokenize::tokenize;
use super::shutting_yard::shutting_yard;
use log::{debug, trace};

fn fold(tokens: Vec<Token>) -> Result<Vec<Token>, String> {
    let mut result = Vec::new();

    let mut prev = None;
    let mut iter = tokens.into_iter().peekable();

    while iter.peek().is_some() {
        let tok = iter.next().unwrap();
        trace!("folding {:?} prev {:?} current folded {:?}", tok, prev, result);
        match tok {
            Token::Operator(Operator::Minus) => {
                if prev.is_none() {
                    match iter.peek() {
                        None => return Err("Error folding the expression".to_string()),
                        Some(t) => {
                            prev = Some(t.clone());
                            trace!("checking next token {:?}", t);
                            match t {
                                Token::Float(f) => {
                                    result.push(Token::Float(-1. * f));
                                    iter.next();
                                },
                                Token::Integer(i) => {
                                    result.push(Token::Integer(-1 * i));
                                    iter.next();
                                },
                                _ => {
                                    result.push(t.clone());
                                    prev = Some(t.clone());
                                    iter.next();
                                }
                            }
                        }
                    }
                } else {
                    match prev.clone().unwrap() {
                        Token::Float(_) | Token::Integer(_) => {
                            result.push(Token::Operator(Operator::Minus));
                            prev = Some(Token::Operator(Operator::Minus));
                        },
                        _ => {
                            match iter.peek() {
                                None => return Err("Error folding the expression".to_string()),
                                Some(t) => {
                                    match t {
                                        Token::Float(f) => {
                                            result.push(Token::Float(-1. * f));
                                            iter.next();
                                        },
                                        Token::Integer(i) => {
                                            result.push(Token::Integer(-1 * i));
                                            iter.next();
                                        },
                                        _ => {
                                            result.push(t.clone());
                                            prev = Some(t.clone());
                                            iter.next();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            _ => {
                result.push(tok.clone());
                prev = Some(tok);
            }
        }
    }

    Ok(result)
}

pub(crate) fn compute(input: String) -> Result<f64, String> {
    debug!("String to evaluate {:?}", input);
    let tokens = tokenize(input)?;
    debug!("parsed string: {:?}", tokens);
    let tokens = fold(tokens)?;
    debug!("folded string: {:?}", tokens);
    let tokens = shutting_yard(tokens)?;
    debug!("re-ordered string: {:?}", tokens);
    let result = evaluate(tokens);
    if let Some(result) = result {
        Ok(result)
    } else {
        Err("Error in computation".to_string())
    }
}

fn evaluate(tokens: Vec<Token>) -> Option<f64> {
    let mut stack = Vec::new();
    for tok in tokens {
        trace!("evaluating {:?} with current stack {:?}", tok, stack);
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