use super::token::Token;

pub(crate) fn shutting_yard(tokens: Vec<Token>) -> Result<Vec<Token>, String> {
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