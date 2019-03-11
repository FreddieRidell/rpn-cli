extern crate termion;

use std::fmt::Display;
use std::fmt;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

struct RPN {
    stack: Vec<f64>,
    input_buffer: String,
}

impl RPN {
    pub fn new() -> RPN {
        RPN {
            stack: Vec::<f64>::new(),
            input_buffer: String::new()
        }
    }

    pub fn add(&mut self) -> Result<&mut Self, &str> {
        Ok(self)
    }

    pub fn sub(&mut self) -> Result<&mut Self, &str> {
        Ok(self)
    }

    pub fn mul(&mut self) -> Result<&mut Self, &str> {
        Ok(self)
    }

    pub fn div(&mut self) -> Result<&mut Self, &str> {
        Ok(self) 
    }

    pub fn pow(&mut self) -> Result<&mut Self, &str> {
        Ok(self) 
    }

    pub fn sqrt(&mut self) -> Result<&mut Self, &str> {
        Ok(self) 
    }

    pub fn resp(&mut self) -> Result<&mut Self, &str> {
        Ok(self) 
    }

    pub fn parse_buffer(&mut self) -> Result<&mut Self, &str> {
        match self.input_buffer.as_str() {
            "+" => self.add(),
            _ => Err("Can not parse buffer")
        }
    }

    pub fn push_buffer(&mut self, c: char) -> &mut Self {
        self.input_buffer.push(c);

        self
    }

    pub fn pop_buffer(&mut self) -> &mut Self {
        self.input_buffer.pop();

        self
    }

    fn stack_indent_width(& self) -> usize {
        self.stack.iter().fold(0, |width, num| {
            let this_width = num.log10() as usize;
            if (this_width > width) {
                this_width
            } else {
                width
            }
        })
    }
}

impl Display for RPN {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.input_buffer)
    }
}


fn main() {
    let mut stack = RPN::new();

    let stdin = stdin();
    let mut stdout = stdout().().unwrap();

    for c in stdin.keys() {

        // Print the key we type...
        match c.unwrap() {
            // Exit.
            Key::Char('q') => break,
            Key::Backspace => { stack.pop_buffer(); },
            Key::Char('\n') => { stack.parse_buffer().unwrap(); },
            Key::Char(c) => { stack.push_buffer(c); }

            _ => println!("Other"),
        }

        write!(stdout, "{}\n", stack);

        // Flush again.
        stdout.flush().unwrap();
    }

    //// Show the cursor again before we exit.
    //write!(stdout, "{}", termion::cursor::Show).unwrap();
}
