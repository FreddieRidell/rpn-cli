use std::fmt;
use std::fmt::Display;
use std::io::{self, BufRead};
use std::slice::Iter;

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Divide,
    Multiply,
    Power,
    Subtract,
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            BinaryOperator::Add => ("+"),
            BinaryOperator::Divide => ("/"),
            BinaryOperator::Multiply => ("*"),
            BinaryOperator::Power => ("^"),
            BinaryOperator::Subtract => ("-"),
        };

        write!(f, "{}", value)
    }
}

#[derive(Debug)]
pub enum Token {
    Drop,
    Operator(BinaryOperator),
    Number(f64),
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Operator(op) => op.fmt(f),

            Token::Drop => write!(f, "_"),

            Token::Number(n) => write!(f, "{}", n.to_string()),
        }
    }
}

impl Token {
    pub fn from_string(s: &str) -> Result<Token, String> {
        match s {
            "+" => Ok(Token::Operator(BinaryOperator::Add)),
            "/" => Ok(Token::Operator(BinaryOperator::Divide)),
            "*" => Ok(Token::Operator(BinaryOperator::Multiply)),
            "^" => Ok(Token::Operator(BinaryOperator::Power)),
            "-" => Ok(Token::Operator(BinaryOperator::Subtract)),

            "_" => Ok(Token::Drop),

            _ => {
                if let Ok(num) = s.parse::<f64>() {
                    Ok(Token::Number(num))
                } else {
                    Err(format!("Can not parse \"{}\"", s))
                }
            }
        }
    }
}
