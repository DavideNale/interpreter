use std::io;

mod ast;
mod lexer;
mod parser;
mod repl;
mod token;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut repl = repl::Repl::new(stdin.lock(), stdout.lock());
    repl.start();
}
