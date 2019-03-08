use std::fmt;
use std::fmt::Display;
use std::io::{self, BufRead};
use std::slice::Iter;

mod token;

pub struct Stack {
    input: Vec<token::Token>,
    working: Vec<token::Token>,
}

impl Stack {
    fn new() -> Stack {
        Stack {
            input: Vec::new(),
            working: Vec::new(),
        }
    }

    fn input_stack(&self) -> impl Iterator<Item = &token::Token> {
        self.input.iter()
    }

    fn working_stack(&self) -> impl Iterator<Item = &token::Token> {
        self.working.iter()
    }

    fn input(&mut self, val: token::Token) -> &mut Self {
        self.input.push(val);

        self
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        for ( i, token ) in self.input_stack().enumerate() {
            let stack_label_width = ((self.input.capacity() as f64).log10()) + 1.0;

            write!(f, "{0:01$}| {2}\n", i, stack_label_width as usize, token);
        };

        write!(f, "" )
    }
}

fn main() {
    let stdin = io::stdin();

    let mut stack = Stack::new();

    for line in stdin.lock().lines() {
        for word in line.unwrap().split(" ") {
            match token::Token::from_string(word) {
                Ok(token) => {
                    stack.input(token);
                }
                Err(msg) => {
                    println!("{}", msg);
                }
            }

            print!("{}", stack);
        }
    }
}
