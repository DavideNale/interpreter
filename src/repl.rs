use std::io::{BufRead, Write};

use crate::{lexer, token};

const PROMPT: &str = ">> ";

pub struct Repl<R: BufRead, W: Write> {
    in_stream: R,
    out_stream: W,
}

impl<R: BufRead, W: Write> Repl<R, W> {
    pub fn new(in_stream: R, out_stream: W) -> Self {
        Self {
            in_stream,
            out_stream,
        }
    }

    pub fn start(&mut self) {
        loop {
            write!(self.out_stream, "{}", PROMPT).expect("Failed to write prompt");
            self.out_stream.flush().expect("Failed to flush stdout");

            let mut input = String::new();
            match self.in_stream.read_line(&mut input) {
                Ok(_) => {
                    let input = input.trim();
                    if input == "exit" {
                        println!("Exiting REPL.");
                        break;
                    }

                    let mut lexer = lexer::Lexer::new(input.to_string());
                    loop {
                        let token = lexer.next_token();
                        if token == token::Token::Eof {
                            break;
                        }
                        writeln!(self.out_stream, "{:?}", token).expect("Failed to write output");
                    }
                }
                Err(_) => {
                    eprintln!("Error reading input");
                    break;
                }
            }
        }
    }
}
