use super::token::Token;
use super::tokenize::tokenize;
use super::shutting_yard::shutting_yard;
use log::{debug, trace};

pub(crate) fn compute(input: String) -> Result<f64, String> {
    debug!("String to evaluate {:?}", input);
    let tokens = tokenize(input);
    debug!("parsed string: {:?}", tokens);
    let tokens = shutting_yard(tokens);
    let tokens = match tokens {
        Ok(tokens) => tokens,
        Err(e) => {
            return Err(e);
        }
    };
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