extern crate termion;

use std::fmt;
use std::fmt::Display;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::terminal_size;
use termion::{clear, color, style};

struct RPN {
    stack: Vec<f64>,
    input_buffer: String,
}

impl RPN {
    pub fn new() -> RPN {
        RPN {
            stack: Vec::<f64>::new(),
            input_buffer: String::new(),
        }
    }

    pub fn binary(&mut self, f: &Fn( &f64, &f64 ) -> f64 ) -> () {
        let stack_size = self.stack.len();

        if let Some(first) = self.stack.get(stack_size - 1) {
            if let Some(second) = self.stack.get(stack_size - 2) {
                let result = f(second, first);
                self.stack.pop();
                self.stack.pop();
                self.stack.push(result);
            }
        }
    }

    pub fn parse_buffer(&mut self) -> () {
        match self.input_buffer.as_str() {
            s => match s.parse::<f64>() {
                Ok(number) => {
                    self.stack.push(number);
                    self.input_buffer = String::new();
                },
                _ => (),
            },
        };
    }

    pub fn push_buffer(&mut self, c: char) -> () {
        self.input_buffer.push(c);
    }

    pub fn pop_buffer(&mut self) -> () {
        self.input_buffer.pop();
    }

    fn stack_indent_width(&self) -> u16 {
        self.stack.iter().fold(0, |width, num| {
            let this_width = num.log10() as u16;
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
        let (width, height) = terminal_size().unwrap();
        let stack_width = self.stack_indent_width();

        for (i, elem) in self.stack.iter().rev().enumerate() {
            let indentation = stack_width - (elem.log10() as u16);

            write!(
                f,
                "{}{}",
                termion::cursor::Goto(1 + indentation, height - ((i as u16) + 9)),
                //termion::clear::CurrentLine,
                elem
            );
        }

        write!(
            f,
            "{}{}",
            termion::cursor::Goto(1, height + 8),
            //termion::clear::CurrentLine,
            self.input_buffer
        )
    }
}

fn main() {
    let mut stack = RPN::new();

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    for c in stdin.keys() {
        write!(
            stdout,
            "{clear}{hide}",
            hide = termion::cursor::Hide,
            clear = termion::clear::All,
            );

        // Print the key we type...
        let response = match c.unwrap() {
            // Exit.
            Key::Char('q') => break,
            Key::Char('+') => stack.binary( &| &x, &y | { x + y } ),
            Key::Char('-') => stack.binary( &| &x, &y | { x - y } ),
            Key::Char('*') => stack.binary( &| &x, &y | { x * y } ),
            Key::Char('/') => stack.binary( &| &x, &y | { x / y } ),
            Key::Backspace => stack.pop_buffer(),
            Key::Char('\n') => stack.parse_buffer(),
            Key::Char(c) => stack.push_buffer(c),

            _ => println!("Other"),
        };

        write!(stdout, "{:?}{}\n", response, stack);

        // Flush again.
        stdout.flush().unwrap();
    }

    //// Show the cursor again before we exit.
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
