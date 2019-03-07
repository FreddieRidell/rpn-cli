use std::fmt;
use std::fmt::Display;
use std::io::{self, BufRead};
use std::slice::Iter;

#[derive(Debug)]
enum Token {
    Add,
    Divide,
    Drop,
    Multiply,
    Power,
    Subtract,
    Number(f64),
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Token::Add => String::from("+"),
            Token::Divide => String::from("/"),
            Token::Drop => String::from("_"),
            Token::Multiply => String::from("*"),
            Token::Power => String::from("^"),
            Token::Subtract => String::from("-"),
            Token::Number(n) => n.to_string(),
        };

        write!(f, "{}", value)
    }
}
impl Token {
    fn from_string(s: &str) -> Result<Token, String> {
        match s {
            "+" => Ok(Token::Add),
            "/" => Ok(Token::Divide),
            "_" => Ok(Token::Drop),
            "*" => Ok(Token::Multiply),
            "^" => Ok(Token::Power),
            "-" => Ok(Token::Subtract),
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

struct Stack {
    input: Vec<Token>,
    working: Vec<Token>,
}

impl Stack {
    fn new() -> Stack {
        Stack {
            input: Vec::new(),
            working: Vec::new(),
        }
    }

    fn input_stack(&self) -> impl Iterator<Item = &Token> {
        self.input.iter()
    }

    fn working_stack(&self) -> impl Iterator<Item = &Token> {
        self.working.iter()
    }

    fn input(&mut self, val: Token) -> &mut Self {
        self.input.push(val);

        self
    }

    //fn push(&mut self, t: Token) -> &mut Self {
    //self.values.push(t);

    //self
    //}

    //fn tail(&self) -> impl Iterator<Item = &Token> {
    //self.values.iter()
    //}

    //fn count(&self) -> usize {
    //self.values.len()
    //}

    //fn peek(&self, i: usize) -> Option<&Token> {
    //let count = self.count();
    //self.values.get(count - i - 1)
    //}

    //fn pop(&mut self) -> Option<Token> {
    //self.values.pop()
    //}

    //fn eval(&mut self) -> &mut Self {

    //let new_val: Result<Token, str> = match self.peek(0) {

    //Some(Token::Add) => {
    //if let Some(Token::Number(n1)) = self.peek(1) {
    //if let Some(Token::Number(n2)) = self.peek(2) {
    //let new_val = Ok(Token::Number(n2 + n1));

    //self.pop();
    //self.pop();
    //self.pop();

    //new_val
    //} else {
    //Err("second value is not a valid number")
    //}
    //} else {
    //Err("first value is not a valid number")
    //}
    //}

    //Some(Token::Subtract) => {
    //if let Some(Token::Number(n1)) = self.peek(1) {
    //if let Some(Token::Number(n2)) = self.peek(2) {
    //let new_val = Ok(Token::Number(n2 - n1));

    //self.pop();
    //self.pop();
    //self.pop();

    //new_val
    //} else {
    //Err("second value is not a valid number")
    //}
    //} else {
    //Err("first value is not a valid number")
    //}
    //}

    //_ => { Err("opperation not yet implemented") },
    //};

    //if let Ok(new_head) = new_val {
    //self.push(new_head);
    //}

    //return self

    //}
}

fn main() {
    let stdin = io::stdin();

    let mut stack = Stack::new();

    for line in stdin.lock().lines() {
        match Token::from_string(line.unwrap().as_str()) {
            Ok(token) => {
                stack.input(token);
            }
            Err(msg) => {
                println!("{}", msg);
            }
        }

        let mut i = 0;
        for token in stack.input_stack() {
            println!("{}. {}", i, token);
            i = i + 1;
        }
    }
}
