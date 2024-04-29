#![allow(dead_code)] // it is what it is, should be removed in the future
mod ast;
mod lexer;
mod parser;
mod repl;

fn main() {
    repl::main();
}
