use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Ord, Clone, Copy)]
pub(crate) enum Operator {
    Plus,
    Minus,
    Div,
    Mul,
    Exp
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
    pub(crate) fn to_prio(&self) -> i32 {
        match *self {
            Operator::Exp => 3,
            Operator::Mul | Operator::Div => 2,
            _ => 1
        }
    }

    pub(crate) fn from(value: char) -> Option<Operator> {
        match value {
            '+' => Some(Operator::Plus),
            '-' => Some(Operator::Minus),
            '/' => Some(Operator::Div),
            '*' => Some(Operator::Mul),
            '^' => Some(Operator::Exp),
            _ => None
        }
    }

    pub(crate) fn evaluate(&self, stack: &mut Vec<f64>) -> Option<f64> {
        if stack.len() < 2 {
            return None;
        }
        match *self {
            Operator::Plus => {
                Some(apply(stack, |a, b| a + b))
            },
            Operator::Minus => {
                Some(apply(stack, |a, b| a - b))
            },
            Operator::Mul => {
                Some(apply(stack,|a, b| a * b))
            },
            Operator::Div => {
                Some(apply(stack,|a, b| a / b))
            },
            Operator::Exp => {
                Some(apply(stack, |a, b| a.powf(b)))
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Token {
    Integer(i64),
    Float(f64),
    Operator(Operator),
    LeftParenthesis,
    RightParenthesis,
}